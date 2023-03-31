use std::sync::Arc;

use ethers::prelude::{abi::AbiDecode, *};

use crate::{
    address_book::{
        UniV2Factory, UniV2Router, UniV2RouterCalls, PANCAKESWAP_FACTORY, SUSHISWAPV2_FACTORY,
        SUSHIV2_FACTORY_ARBITRUM, TRADERJOE_FACTORY_ARBITRUM, UNISWAPV2_FACTORY, UNISWAPV3_FACTORY,
        VVS_FACTORY,
    },
    timestamp_print,
};

use crate::pair::Pair;

use colored::*;

use eyre::Result;

abigen!(
    UniV2Factory,
    r#"[
        event PairCreated(address indexed token0, address indexed token1, address pair, uint)
        event PoolCreated(address indexed token0, address indexed token1, uint24 indexed fee, int24 tickSpacing, address pool)
    ]"#
);

#[allow(dead_code)]
pub struct Dex {
    pub chain: String,
    pub name: String,
    pub factory_address: Address,
    pub router_address: Address,
    factory: UniV2Factory<Provider<Http>>,
    router: UniV2Router<Provider<Http>>,
}

impl Dex {
    pub fn new(
        middleware: Arc<Provider<Http>>,
        factory_address: Address,
        router_address: Address,
        chain: String,
    ) -> Self {
        let factory = UniV2Factory::new(factory_address, Arc::clone(&middleware));
        let router = UniV2Router::new(router_address, Arc::clone(&middleware));

        let name = Self::get_dex_name(factory_address);

        Self {
            chain,
            name,
            factory_address,
            router_address,
            factory,
            router,
        }
    }

    /// A quick way to decode tx hex data.
    pub async fn decode_router_tx_data(&self, tx_data: String) {
        let calldata: Bytes = tx_data.parse().unwrap();
        let decoded = UniV2RouterCalls::decode(&calldata).unwrap();
        println!("Decoded dex tx: {:?}", decoded);
    }

    /// Attempts to retrieve the total pairs created from the dex's factory.
    /*
    pub async fn get_pairs(&self) {
        println!("Calling allPairsLength from {}", self.factory_address);
        match self.factory.all_pairs_length().call().await {
            Ok(result) => {
                println!("   ~ [PASS] Total pairs: {:?}", result)
            }
            Err(e) => {
                println!("   ~ [FAIL] Total pairs: {:?}", e)
            }
        }
    }
    */
    pub fn get_dex_name(contract_address: Address) -> String {
        let address_str = format!("{:#066x}", contract_address);

        let dex_name = match address_str.as_str() {
            UNISWAPV3_FACTORY => "Uniswap V3".to_string(),
            UNISWAPV2_FACTORY => "Uniswap V2".to_string(),
            SUSHISWAPV2_FACTORY => "Sushiswap V2".to_string(),
            SUSHIV2_FACTORY_ARBITRUM => "Sushiswap V2".to_string(),
            VVS_FACTORY => "VVS Finance".to_string(),
            PANCAKESWAP_FACTORY => "Pancakeswap".to_string(),
            TRADERJOE_FACTORY_ARBITRUM => "Trader Joe".to_string(),
            _default => "UniswapV2 Like".to_string(),
        };
        dex_name
    }

    pub async fn token_stream_pairs_created(
        self,
        ws: Arc<Provider<Ws>>,
        http: Arc<Provider<Http>>,
        token_address: Address,
    ) -> Result<()> {
        println!(
            "----------------------- {} | {} -----------------------",
            self.name, self.chain
        );
        timestamp_print!(
            Color::BrightBlue,
            Some(false),
            format!(
                "[{}] Listening for PairCreated events, on token {} from {}",
                token_address, self.name, self.factory_address
            )
        );
        let contract = UniV2Factory::new(self.factory_address, ws.clone());
        let events = contract.events();
        let mut stream = events.stream().await?.with_meta();

        let dex = Arc::new(self);
        while let Some(Ok((event, meta))) = stream.next().await {
            if event.token_0 == token_address || event.token_1 == token_address {
                timestamp_print!(
                    Color::Green,
                    Some(false),
                    format!(
                        "[PAIR][{}] PairCreated detected: {:#066x}",
                        dex.name, event.pair
                    )
                );
                let pair =
                    Pair::new(event.pair, event.token_0, event.token_1, &dex, http.clone()).await;

                let mut pair = Arc::new(pair);
                let ws_clone = ws.clone();
                let http_clone = http.clone();
                tokio::spawn(async move {
                    pair.stream_add_liquidity(ws_clone, http_clone, meta.block_number)
                        .await
                        .unwrap();
                });
                timestamp_print!(
                    Color::BrightBlue,
                    Some(false),
                    format!(
                        "[PAIR][{}] Listening for new PairCreated events, from {}",
                        dex.name, dex.factory_address
                    )
                );
            }
        }
        Ok(())
    }

    /// Streams the "PairCreated" event from the `factory_address`.
    pub async fn stream_pairs_created(
        self,
        ws: Arc<Provider<Ws>>,
        http: Arc<Provider<Http>>,
    ) -> Result<()> {
        println!(
            "----------------------- {} | {} -----------------------",
            self.name, self.chain
        );
        timestamp_print!(
            Color::BrightBlue,
            Some(false),
            format!(
                "[{}] Listening for PairCreated events, from {}",
                self.name, self.factory_address
            )
        );
        let contract = UniV2Factory::new(self.factory_address, ws.clone());
        let events = contract.events();
        let mut stream = events.stream().await?.with_meta();

        let dex = Arc::new(self);
        while let Some(Ok((event, meta))) = stream.next().await {
            timestamp_print!(
                Color::Green,
                Some(false),
                format!(
                    "[PAIR][{}] PairCreated detected: {:#066x}",
                    dex.name, event.pair
                )
            );
            let pair =
                Pair::new(event.pair, event.token_0, event.token_1, &dex, http.clone()).await;

            let mut pair = Arc::new(pair);
            let ws_clone = ws.clone();
            let http_clone = http.clone();
            tokio::spawn(async move {
                pair.stream_add_liquidity(ws_clone, http_clone, meta.block_number)
                    .await
                    .unwrap();
            });
            timestamp_print!(
                Color::BrightBlue,
                Some(false),
                format!(
                    "[PAIR][{}] Listening for new PairCreated events, from {}",
                    dex.name, dex.factory_address
                )
            );
        }
        Ok(())
    }
}
