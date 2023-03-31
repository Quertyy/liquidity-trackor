use ethers::prelude::*;

pub fn address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

pub fn wei_to_float(input: u128) -> f64 {
    input as f64 / 1_000_000_000_000_000_000.0
}

pub fn token_amount(input: u128, decimal: u8) -> f64 {
    input as f64 / 10.0_f64.powi(decimal as i32)
}

pub fn weth_price() -> f64 {
    3000.0
}

pub fn to_1e18(input: u64) -> U256 {
    let ether: U256 = U256::exp10(18);
    let parsed: U256 = input.into();
    parsed * ether
}
