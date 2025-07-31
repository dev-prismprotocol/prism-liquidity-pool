// programs/liquidity_pool/src/state/pool.rs
use anchor_lang::prelude::*;

#[account]
pub struct LiquidityPool {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub exchange_rate: u64,
    pub sol_reserve: u64,
    pub token_reserve: u64,
    pub is_active: bool,
    pub created_at: i64,
    pub fee_basis_points: u16,
    pub bump: u8,
    pub total_volume_sol: u64,
    pub total_volume_token: u64,
    pub trade_count: u64,
    pub last_trade_timestamp: i64,
    pub current_price: f64,
}

impl LiquidityPool {
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1 + 8 + 2 + 1 + 8 + 8 + 8 + 8 + 8 + 64;

    pub fn update_trade_stats(&mut self, sol_amount: u64, token_amount: u64, timestamp: i64) {
        self.total_volume_sol = self.total_volume_sol.saturating_add(sol_amount);
        self.total_volume_token = self.total_volume_token.saturating_add(token_amount);
        self.trade_count = self.trade_count.saturating_add(1);
        self.last_trade_timestamp = timestamp;
        self.current_price = if token_amount > 0 {
            (sol_amount as f64) / (token_amount as f64)
        } else {
            0.0
        };
    }
}