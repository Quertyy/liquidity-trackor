use std::sync::Arc;

use ethers::prelude::{k256::ecdsa::SigningKey, *};
use eyre::Result;

use reqwest::Error;

use crate::timestamp_print;
use colored::*;

#[derive(Clone)]
pub struct Token {
    pub address: Address,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub is_verified: bool,
}

impl Token {
    pub async fn new(address: Address, provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) -> Result<Self> {
        abigen!(ERC20, "src/abi/ERC20.json");
        let provider = provider.clone();
        let name = ERC20::new(address, provider.clone())
            .name()
            .call()
            .await
            .unwrap();
        let symbol = ERC20::new(address, provider.clone())
            .symbol()
            .call()
            .await
            .unwrap();
        let decimals = ERC20::new(address, provider.clone())
            .decimals()
            .call()
            .await
            .unwrap();

        Ok(Self {
            address,
            name,
            symbol,
            decimals,
            is_verified: false,
        })
    }

    pub async fn verified_contract(&mut self, chain: String) -> Result<bool, Error> {
        if self.is_verified {
            return Ok(true);
        }
        let explorer = Token::get_explorer(chain);
        let api_key = std::env::var("ETHERSCAN_API_KEY").expect("missing API KEY");
        let url = format!(
            "https://api.{}api?module=contract&action=getabi&address={:#066x}&apikey={}",
            explorer, 
            self.address, 
            api_key
        );

        let verified: bool;
        let status: Color;
        let responsed = reqwest::get(&url).await?.text().await?;
        if responsed.contains("NOTOK") {
            status = Color::Red;
            verified = false;
        } else {
            status = Color::Green;
            verified = true;
        }

        timestamp_print!(status, Some(false), format!("[TOKEN] {} | Contrat verified: {}", self.name, verified));
        if verified {
            self.is_verified = true;
            timestamp_print!(Color::Blue, Some(false), format!("[TOKEN] Contract code: https://{}address/{:#066x}#code", explorer, self.address));
        }
        Ok(verified)
    }

    fn get_explorer(chain: String) -> String {
        match chain.to_lowercase().as_str() {
            "ethereum" => "etherscan.io/".to_string(),
            "ropsten" => "ropsten.etherscan.io/".to_string(),
            "rinkeby" => "rinkeby.etherscan.io/".to_string(),
            "goerli" => "goerli.etherscan.io/".to_string(),
            "kovan" => "kovan.etherscan.io/".to_string(),
            "bsc" => "bscscan.com/".to_string(),
            "arbitrum" => "arbiscan.io/".to_string(),
            "polygon" => "polygonscan.com/".to_string(),
            "avalanche" =>  "snowtrace.io/".to_string(),
            _ => "etherscan.io/".to_string(),
        }
    }

    
}