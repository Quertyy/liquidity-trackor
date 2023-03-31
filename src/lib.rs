pub mod helpers;
pub mod block_scanner;
pub mod mempool;
pub mod address_book;
pub mod dex;
pub mod pair;
pub mod token;
pub mod alert;
pub mod contracts;

use contracts::get_contracts_data;
use helpers::address;

use crate::dex::Dex;

use std::sync::Arc;
use ethers::prelude::*;
use tokio::signal::ctrl_c;
use clap::Parser;
use colored::*;

#[macro_export]
macro_rules! timestamp_print {
    ($color: expr, $large: expr, $message: expr) => {
        let formatted_message = if let Some(true) = $large {
            $message.chars().map(|c| format!("{}", c).bold().to_string()).collect::<String>()
        } else {
            $message.clone()
        };
        println!(
            "{} {} {}",
            chrono::Local::now()
                .format("[%Y-%m-%d]")
                .to_string()
                .color($color),
            chrono::Local::now()
                .format("[%H:%M:%S]")
                .to_string()
                .color($color),
            formatted_message.color($color)
        );
    };
}

#[derive(Debug, Clone, Copy)]
pub enum Chain {
    Ethereum,
    Arbitrum,
    Polygon,
    Bsc,
    Fantom,
    Avalanche,
    Cronos,
}

impl std::str::FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ethereum" => Ok(Chain::Ethereum),
            "arbitrum" => Ok(Chain::Arbitrum),
            "polygon" => Ok(Chain::Polygon),
            "bsc" => Ok(Chain::Bsc),
            "fantom" => Ok(Chain::Fantom),
            "avalanche" => Ok(Chain::Avalanche),
            "cronos" => Ok(Chain::Cronos),
            _ => Err(String::from("Invalid chain name")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Chain that you want to track
   #[arg(short, long, default_value = "ethereum")]
   chain: Chain,
   /// Monitor block creation
   #[arg(short, long, default_value = "false")]
   block: bool,
   /// JSON file with addresses to monitor
   #[arg(short, long, default_value = "src/json/dex_contracts.json")]
   json: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    #[allow(dead_code)]
    pub http: Arc<Provider<Http>>,
    #[allow(dead_code)]
    pub wss: Arc<Provider<Ws>>,
}

impl Config {
    pub async fn new(chain: &str) -> Self {
        
        
        let network = std::env::var(format!("NETWORK_RPC_{}", chain)).expect("missing NETWORK_RPC");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(provider);

        let ws_network = std::env::var(format!("NETWORK_WSS_{}", chain)).expect("missing NETWORK_WSS");
        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_network).await.unwrap();
        
        Self {
            http: middleware,
            wss: Arc::new(ws_provider),
        }
    }

    pub async fn create_dex(&self, factory: Address, router: Address, chain: String) -> Dex {
        Dex::new(self.http.clone(), factory, router, chain)
    }
}

fn conf_arg() -> (String, bool, String) {
    let args = Args::parse();
    let key;
    match args.chain {
        Chain::Ethereum => {
            key = "ETHEREUM"
        }
        Chain::Arbitrum => {
            key = "ARBITRUM"
        }
        Chain::Polygon => {
            key = "POLYGON"
        }
        Chain::Bsc => {
            key = "BSC"
        }
        Chain::Fantom => {
            key = "FANTOM"
        }
        Chain::Avalanche => {
            key = "AVALANCHE"
        }
        Chain::Cronos => {
            key = "CRONOS"
        }
    }
    (key.to_string(), args.block, args.json)
}

async fn config_task(dex_router: H160, dex_factory: H160, chain: String) {
    let config = Config::new(chain.as_str()).await;
    tokio::spawn(async move {
        let dex = config.create_dex(dex_factory, dex_router, chain.clone()).await;
        dex.stream_pairs_created(config.wss, config.http).await.unwrap();
    });
}

pub async fn run() {    
    let (chain, block_monitoring, path) = conf_arg();
    let chain_block = chain.clone();

    let data = get_contracts_data(path);

    if block_monitoring {
        timestamp_print!(Color::Green, Some(true), format!("Starting block monitor"));
        let config = Config::new(chain_block.as_str()).await;
        tokio::spawn(async move {
            block_scanner::loop_blocks(config.http).await;
        });
    }

    if let Some(chain_data) = data.get(chain.as_str()) {
        for (_, addresses) in &chain_data.addresses {
            config_task(
                address(addresses.router.as_str()), 
                address(addresses.factory.as_str()), 
                chain.clone()
            ).await;
        }
    } else {
        println!("Chain not supported");
    }    

    let (tx, rx) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        ctrl_c().await.expect("failed to listen for event");
        let _ = tx.send(());
    });

    let _ = rx.await;
}