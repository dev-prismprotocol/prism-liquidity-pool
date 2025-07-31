// programs/liquidity_pool/src/utils/validation.rs
use anchor_lang::prelude::*;
use crate::error::LiquidityPoolError;
use crate::state::constants::*;

pub fn validate_swap_params(amount: u64, reserve: u64) -> Result<()> {
    require!(amount >= MIN_SWAP_AMOUNT, LiquidityPoolError::InvalidSwapAmount);
    require!(amount <= reserve, LiquidityPoolError::InsufficientLiquidity);
    Ok(())
}

pub fn validate_fee(fee_basis_points: u16) -> Result<()> {
    require!(
        fee_basis_points <= MAX_FEE_BASIS_POINTS,
        LiquidityPoolError::FeeExceedsMaximum
    );
    Ok(())
}

pub fn validate_reserves(sol_reserve: u64, token_reserve: u64) -> Result<()> {
    require!(sol_reserve > 0 || token_reserve > 0, LiquidityPoolError::InvalidReserves);
    Ok(())
}

pub fn validate_liquidity_params(sol_amount: u64, token_amount: u64) -> Result<()> {
    require!(sol_amount > 0 || token_amount > 0, LiquidityPoolError::InvalidReserves);
    Ok(())
}