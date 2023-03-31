use std::sync::Arc;

use ethers::{
    abi::AbiDecode,
    providers::{Middleware, Provider, StreamExt, TransactionStream, Ws}
};

use crate::address_book::UniV2RouterCalls;
use crate::timestamp_print;

use colored::*;

pub async fn loop_mempool(ws_provider: Arc<Provider<Ws>>) {
    let tx_hash_stream = ws_provider.subscribe_pending_txs().await.unwrap();
    let mut tx_stream = TransactionStream::new(&ws_provider, tx_hash_stream, 256);

    timestamp_print!(Color::White, Some(false), format!("---------- MONITORING MEMPOOL ----------"));
    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                timestamp_print!(Color::BrightBlue, Some(false), format!("Transaction: {:#?}", tx));
                timestamp_print!(Color::BrightBlue, Some(false), format!("Router Call: {:#?}", decoded));
            }
        }
    }
}