use std::sync::Arc;

use ethers::prelude::*;

use crate::alert::alert;
use crate::dex::Dex;
use crate::helpers::{token_amount, wei_to_float};
use crate::timestamp_print;
use crate::token::Token;

use colored::*;
use eyre::Result;

pub struct Pair {
    pub dex: Arc<Dex>,
    pub address: Address,
    pub token0: Token,
    pub token1: Token,
    #[warn(dead_code)]
    new_token: Token,
    pub reserves: (U256, U256),
}

impl Pair {
    pub async fn new(
        address: Address,
        token0_address: Address,
        token1_address: Address,
        dex: &Arc<Dex>,
        provider: Arc<Provider<Http>>,
    ) -> Pair {
        let mut token0: Token = Token::new(token0_address, provider.clone()).await.unwrap();
        let mut token1: Token = Token::new(token1_address, provider.clone()).await.unwrap();
        timestamp_print!(
            Color::Green,
            Some(false),
            format!(
                "TokenIn: {} | {} | {:#066x}",
                token0.name, token0.symbol, token0.address
            )
        );
        timestamp_print!(
            Color::Green,
            Some(false),
            format!(
                "TokenOut: {} | {} | {:#066x}",
                token1.name, token1.symbol, token1.address
            )
        );

        let new_token: Token;
        if token0.symbol == "WETH" {
            new_token = token1.clone();
            if dex.verify { token1.verified_contract(dex.chain.clone()).await.unwrap() ;}
        } else {
            if dex.verify { token0.verified_contract(dex.chain.clone()).await.unwrap() ;}
            new_token = token0.clone();
        }

        Pair {
            dex: Arc::clone(dex),
            address,
            token0,
            token1,
            new_token,
            reserves: (0.into(), 0.into()),
        }
    }

    pub async fn get_reserves(&mut self, provider: Arc<Provider<Http>>) -> Result<()> {
        abigen!(
            IUniswapV2Pair,
            r#"[
                function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
            ]"#,
        );
        let pair = IUniswapV2Pair::new(self.address, provider.clone());
        let (reserve0, reserve1, _) = pair.get_reserves().call().await?;
        self.reserves = (reserve0.into(), reserve1.into());
        timestamp_print!(
            Color::Green,
            Some(false),
            format!(
                "[PAIR][LIQ] Current reserves: {} {} | {} {}",
                wei_to_float(self.reserves.0.as_u128()),
                self.token0.symbol,
                wei_to_float(self.reserves.1.as_u128()),
                self.token1.symbol
            )
        );
        Ok(())
    }

    /// Streams the "Mint" event from the pair address -> addLiquidity
    pub async fn stream_add_liquidity(
        self: &mut Arc<Self>,
        ws: Arc<Provider<Ws>>,
        http: Arc<Provider<Http>>,
        block_number: U64,
    ) -> Result<()> {
        abigen!(
            UniV2Pair,
            r#"[
                event Mint(address indexed sender, uint amount0, uint amount1)
            ]"#,
        );

        //abigen!(
        //    UniV3Pair,
        //    r#"[
        //        event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1)
        //    ]"#,
        //);

        let contract = UniV2Pair::new(self.address, ws.clone());

        let events = contract.event::<MintFilter>();
        let block_events = contract
            .event::<MintFilter>()
            .from_block(block_number)
            .query()
            .await?;

        let mut amount0: U256 = 0.into();
        let mut amount1: U256 = 0.into();

        if !block_events.is_empty() {
            amount0 = block_events[0].amount_0;
            amount1 = block_events[0].amount_1;
        } else {
            let mut stream = events.stream().await?.take(1);
            timestamp_print!(
                Color::BrightCyan,
                Some(false),
                format!(
                    "[PAIR] Listening for Mint events on pair {}, from router {}",
                    self.address, self.dex.router_address
                )
            );

            while let Some(Ok(f)) = stream.next().await {
                amount0 = f.amount_0;
                amount1 = f.amount_1;
            }
        }
        let amount_0 = token_amount(amount0.as_u128(), self.token0.decimals);
        let amount_1 = token_amount(amount1.as_u128(), self.token1.decimals);
        timestamp_print!(
            Color::Green,
            Some(true),
            format!(
                "[PAIR][{}][FOUND] AddLiquidity detected on pair: {:#066x}",
                self.dex.name, self.address
            )
        );
        timestamp_print!(
            Color::Green,
            Some(false),
            format!("[PAIR][LIQ] TokenIn: {:?} {}", amount_0, self.token0.symbol)
        );
        timestamp_print!(
            Color::Green,
            Some(false),
            format!(
                "[PAIR][LIQ] TokenOut: {:?} {}",
                amount_1, self.token1.symbol
            )
        );
        let self_mut = Arc::get_mut(self).unwrap();
        self_mut.get_reserves(http.clone()).await.unwrap();

        if self_mut.dex.verify {
            self_mut
            .new_token
            .verified_contract(self_mut.dex.chain.clone())
            .await
            .unwrap();
        }

        let pair = self.clone();
        timestamp_print!(
            Color::Blue,
            Some(false),
            format!(
                "[PAIR] Chart: https://dexscreener.com/{}/{:#066x}",
                self.dex.chain.to_lowercase(),
                self.address
            )
        );
        if self.dex.alert { alert(pair, amount_0, amount_1).await.unwrap() }
        Ok(())
    }
}
