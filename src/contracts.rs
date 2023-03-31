use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ContractAddresses {
    pub router: String,
    pub factory: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChainData {
    #[serde(flatten)]
   pub addresses: HashMap<String, ContractAddresses>,
}

pub fn get_contracts_data(path: String) -> HashMap<String, ChainData> {
    let path = std::path::Path::new(path.as_str());
    let content = std::fs::read_to_string(path).expect("Can't read file");
    let data: HashMap<String, ChainData> =
        serde_json::from_str(&content).expect("JSON deserialization failed");
    data
}