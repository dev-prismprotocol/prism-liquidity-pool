// programs/liquidity_pool/src/state/constants.rs
pub const MAX_FEE_BASIS_POINTS: u16 = 1000;
pub const MIN_EXCHANGE_RATE: u64 = 1;
pub const MAX_EXCHANGE_RATE: u64 = u64::MAX;
pub const MIN_SWAP_AMOUNT: u64 = 1;
pub const POOL_SEED: &[u8] = b"pool";
pub const DEX_NAME: &str = "Liquidity Pool";
pub const PROGRAM_NAME: &str = "Liquidity Pool";
pub const DEX_TYPE: &str = "AMM";
pub const POOL_TYPE: &str = "AMM";
pub const TRANSACTION_TYPE: &str = "swap";
pub const PROGRAM_VERSION: &str = "1.0.0";

pub fn get_network_name() -> String {
    #[cfg(feature = "mainnet")]
    return "mainnet-beta".to_string();

    #[cfg(feature = "devnet")]
    return "devnet".to_string();

    "localnet".to_string()
}