// programs/liquidity_pool/src/instructions/create_pool.rs
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use crate::state::{LiquidityPool, DEX_NAME, POOL_TYPE, PROGRAM_VERSION};
use crate::events::*;
use crate::error::LiquidityPoolError;

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(
        init,
        payer = pool_creator,
        space = LiquidityPool::SPACE,
        seeds = [b"pool", token_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(
        constraint = program_authority.key() == crate::ID @ LiquidityPoolError::UnauthorizedAccess
    )]
    pub program_authority: Signer<'info>,

    #[account(mut)]
    pub pool_creator: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreatePool>,
    virtual_token_reserve: Option<u64>,
    virtual_sol_reserve: Option<u64>,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let pool_creator = &ctx.accounts.pool_creator;
    let current_time = Clock::get()?.unix_timestamp;

    const DEFAULT_VIRTUAL_SOL: u64 = 30_000_000_000;
    const DEFAULT_VIRTUAL_TOKEN: u64 = 1_073_000_000_000_000;

    let final_virtual_sol = virtual_sol_reserve.unwrap_or(DEFAULT_VIRTUAL_SOL);
    let final_virtual_token = virtual_token_reserve.unwrap_or(DEFAULT_VIRTUAL_TOKEN);

    pool.authority = pool_creator.key();
    pool.token_mint = ctx.accounts.token_mint.key();
    pool.sol_reserve = final_virtual_sol;
    pool.token_reserve = final_virtual_token;
    pool.exchange_rate = final_virtual_token
        .checked_mul(1_000_000)
        .unwrap_or(u64::MAX)
        .checked_div(final_virtual_sol)
        .unwrap_or(1_000_000_000_000);
    pool.is_active = true;
    pool.created_at = current_time;
    pool.fee_basis_points = 0;
    pool.bump = *ctx.bumps.get("pool").unwrap();
    pool.total_volume_sol = 0;
    pool.total_volume_token = 0;
    pool.trade_count = 0;
    pool.last_trade_timestamp = 0;
    pool.current_price = (final_virtual_sol as f64) / (final_virtual_token as f64);

    emit!(PoolCreated {
        pool: pool.key(),
        authority: pool_creator.key(),
        token_mint: ctx.accounts.token_mint.key(),
        initial_exchange_rate: pool.exchange_rate,
        initial_sol: final_virtual_sol,
        initial_tokens: final_virtual_token,
        dex_name: DEX_NAME.to_string(),
        pool_type: POOL_TYPE.to_string(),
        version: PROGRAM_VERSION.to_string(),
    });

    Ok(())
}