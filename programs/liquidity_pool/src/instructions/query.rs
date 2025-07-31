// programs/liquidity_pool/src/instructions/query.rs
use anchor_lang::prelude::*;
use crate::state::LiquidityPool;
use crate::utils::math::{calculate_sol_to_tokens, calculate_tokens_to_sol};

#[derive(Accounts)]
pub struct GetPoolInfo<'info> {
    #[account(
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,
}

#[derive(Accounts)]
pub struct CalculateSwap<'info> {
    #[account(
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,
}

pub fn get_pool_info_handler(ctx: Context<GetPoolInfo>) -> Result<()> {
    let pool = &ctx.accounts.pool;

    msg!("Pool Info:");
    msg!("- Address: {}", pool.key());
    msg!("- Token Mint: {}", pool.token_mint);
    msg!("- Authority: {}", pool.authority);
    msg!("- Exchange Rate: {}", pool.exchange_rate);
    msg!("- SOL Reserve: {}", pool.sol_reserve);
    msg!("- Token Reserve: {}", pool.token_reserve);
    msg!("- Active: {}", pool.is_active);
    msg!("- Created At: {}", pool.created_at);

    Ok(())
}

pub fn calculate_swap_handler(
    ctx: Context<CalculateSwap>,
    input_amount: u64,
    token_to_sol: bool,
) -> Result<()> {
    let pool = &ctx.accounts.pool;

    let output_amount = if token_to_sol {
        calculate_tokens_to_sol(input_amount, pool.exchange_rate)?
    } else {
        calculate_sol_to_tokens(input_amount, pool.exchange_rate)?
    };

    msg!("Swap Calculation:");
    msg!("- Input: {}", input_amount);
    msg!("- Output: {}", output_amount);
    msg!("- Direction: {}", if token_to_sol { "Tokens → SOL" } else { "SOL → Tokens" });

    Ok(())
}