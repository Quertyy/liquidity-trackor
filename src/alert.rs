use webhook::client::{WebhookClient, WebhookResult};

use std::sync::Arc;

use crate::pair::Pair;

pub async fn alert(pair: Arc<Pair>, amount_in: f64, amount_out: f64) -> WebhookResult<()> {

    let webhook = get_webhook(pair.clone());
    let client = WebhookClient::new(&webhook);

    client.send(|message| message
        .content("@everyone")
        .username("Liquidity Alert Bot")
        .embed(|embed| embed
            .title(format!("{}", pair.dex.name).as_str())
            .description(format!("Liquidity added on pair {}/{}", pair.token0.symbol, pair.token1.symbol).as_str())
            .field("AmountIn", format!("{} {}", amount_in, pair.token0.symbol).as_str(), false)
            .field("AmountOut", format!("{} {}", amount_out, pair.token1.symbol).as_str(), false)
            .field("Chart", format!("https://dexscreener.com/{}/{:#066x}", pair.dex.chain.to_lowercase().as_str(), pair.address).as_str(), false)
        )
    ).await?;

    Ok(())
}

fn get_webhook(pair: Arc<Pair>) -> String {
    match pair.dex.chain.as_str() {
        "ETHEREUM" => std::env::var("DISCORD_WEBHOOK_ETHEREUM").expect("missing DISCORD_WEBHOOK_ETHEREUM"),
        "BSC" => std::env::var("DISCORD_WEBHOOK_BSC").expect("missing DISCORD_WEBHOOK_BSC"),
        "POLYGON" => std::env::var("DISCORD_WEBHOOK_POLYGON").expect("missing DISCORD_WEBHOOK_POLYGON"),
        "ARBITRUM" => std::env::var("DISCORD_WEBHOOK_ARBITRUM").expect("missing DISCORD_WEBHOOK_ARBITRUM"),
        _ => panic!("chain not supported"),
    }
}