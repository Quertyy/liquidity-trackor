use webhook::client::{WebhookClient, WebhookResult};

use std::sync::Arc;

use crate::pair::Pair;

pub async fn alert(pair: Arc<Pair>, amount_in: f64, amount_out: f64) -> WebhookResult<()> {

    let chain_str = pair.dex.chain.to_uppercase();
    let webhoook_env = format!("DISCORD_WEBHOOK_{}", chain_str);
    let role_env = format!("ROLE_ID_{}", chain_str);
    let webhook = std::env::var(webhoook_env).expect("missing WEBHOOK");
    let client = WebhookClient::new(&webhook);

    let role_id = std::env::var(role_env).expect("missing ROLE_ID");

    client.send(|message| message
        .content(format!("<@&{}>", role_id).as_str())
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