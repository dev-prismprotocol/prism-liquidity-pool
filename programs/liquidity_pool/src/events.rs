// programs/liquidity_pool/src/events.rs
use anchor_lang::prelude::*;

#[event]
pub struct SwapExecuted {
    #[index]
    pub pool: Pubkey,
    #[index]
    pub trader: Pubkey,
    #[index]
    pub token_mint: Pubkey,
    pub dex_name: String,
    pub trade_direction: String,
    pub token_amount: u64,
    pub sol_amount: u64,
    pub exchange_rate: f64,
    pub timestamp: i64,
    pub transaction_type: String,
    pub pool_type: String,
    pub slippage: f64,
    pub fee_amount: u64,
    pub program_version: String,
    pub network: String,
}

#[event]
pub struct PoolCreated {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub initial_exchange_rate: u64,
    pub initial_sol: u64,
    pub initial_tokens: u64,
    pub dex_name: String,
    pub pool_type: String,
    pub version: String,
}

#[event]
pub struct LiquidityAdded {
    pub pool: Pubkey,
    pub liquidity_provider: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub dex_name: String,
    pub transaction_type: String,
}

#[event]
pub struct LiquidityRemoved {
    pub pool: Pubkey,
    pub liquidity_provider: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub dex_name: String,
    pub transaction_type: String,
}

#[event]
pub struct ExchangeRateUpdated {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub old_rate: u64,
    pub new_rate: u64,
}

#[event]
pub struct PoolToggled {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub is_active: bool,
}

#[event]
pub struct PoolClosed {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub final_sol_reserve: u64,
    pub final_token_reserve: u64,
}