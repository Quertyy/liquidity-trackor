use ethers::prelude::*;

// ARBITRUM
#[allow(dead_code)]
pub(crate) const WETH_ARBI: &str = "0x82af49447d8a07e3bd95bd0d56f35241523fbab1";
#[allow(dead_code)]
pub(crate) const USDC_ARBI: &str = "0xff970a61a04b1ca14834a43f5de4533ebddb5cc8";

pub(crate) const UNISWAPV3_FACTORY: &str = "0x1f98431c8ad98523631ae4a59f267346ea31f984";
#[allow(dead_code)]
pub(crate) const UNISWAPV3_ROUTER: &str = "0xe592427a0aece92de3edee1f18e0157c05861564";
pub(crate) const SUSHIV2_FACTORY_ARBITRUM: &str = "0xc35dadb65012ec5796536bd9864ed8773abc74c4";
#[allow(dead_code)]
pub(crate) const SUSHISWAPV2_ROUTER_ARBITRUM: &str = "0x1b02da8cb0d097eb8d57a175b88c7d8b47997506";
#[allow(dead_code)]
pub(crate) const TRADERJOE_ROUTER_ARBITRUM: &str = "0x1886D09C9Ade0c5DB822D85D21678Db67B6c2982";
#[allow(dead_code)]
pub(crate) const TRADERJOE_FACTORY_ARBITRUM: &str = "0x7BFd7192E76D950832c77BB412aaE841049D8D9B";


// ETHEREUM
#[allow(dead_code)]
pub(crate) const USDC_MAINNET: &str = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
#[allow(dead_code)]
pub(crate) const WETH_MAINNET: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";

pub(crate) const UNISWAPV2_FACTORY: &str = "0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f";
#[allow(dead_code)]
pub(crate) const UNISWAPV2_ROUTER: &str = "0x7a250d5630b4cf539739df2c5dacb4c659f2488d";

pub(crate) const SUSHISWAPV2_FACTORY: &str = "0xc0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac";
#[allow(dead_code)]
pub(crate) const SUSHISWAPV2_ROUTER: &str = "0xd9e1ce17f2641f24ae83637ab66a2cca9c378b9f";

pub(crate) const PANCAKESWAP_FACTORY: &str = "0x1097053fd2ea711dad45caccc45eff7548fcb362";

// AVALANCHE
#[allow(dead_code)]
pub(crate) const WAVAX: &str = "0xb31f66aa3c1e785363f0875a1b74e27b85fd66c7";

// BSC


// FANTOM
#[allow(dead_code)]
pub(crate) const SPOOKY_SWAP_ROUTER: &str = "0xF491e7B69E4244ad4002BC14e878a34207E38c29";
#[allow(dead_code)]
pub(crate) const SPOOKY_SWAP_FACTORY: &str = "0x152eE697f2E276fA89E96742e9bB9aB1F2E61bE3";


// CRONOS
pub(crate) const VVS_FACTORY: &str = "0x3B44B2a187a7b3824131F8db5a74194D0a42Fc15";
#[allow(dead_code)]
pub(crate) const VVS_ROUTER: &str = "0x145863Eb42Cf62847A6Ca784e6416C1682b1b2Ae";


abigen!(UniV2Router, "src/abi/UniV2Router.json");
abigen!(UniV2Factory, "src/abi/UniV2Factory.json");
abigen!(LpPair, "src/abi/LpPair.json");