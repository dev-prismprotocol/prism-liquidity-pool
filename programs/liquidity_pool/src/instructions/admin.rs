// programs/liquidity_pool/src/instructions/admin.rs
use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::*;
use crate::events::*;
use crate::utils::auth::check_authority;

#[derive(Accounts)]
pub struct UpdateExchangeRate<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TogglePool<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePool<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump,
        close = authority
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn update_exchange_rate_handler(
    ctx: Context<UpdateExchangeRate>,
    new_rate: u64,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    check_authority(&ctx.accounts.authority.key(), &pool.authority)?;
    require!(new_rate > 0, LiquidityPoolError::InvalidExchangeRate);

    let old_rate = pool.exchange_rate;
    pool.exchange_rate = new_rate;

    emit!(ExchangeRateUpdated {
        pool: pool.key(),
        authority: ctx.accounts.authority.key(),
        old_rate,
        new_rate,
    });

    Ok(())
}

pub fn toggle_pool_handler(ctx: Context<TogglePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    check_authority(&ctx.accounts.authority.key(), &pool.authority)?;

    pool.is_active = !pool.is_active;

    emit!(PoolToggled {
        pool: pool.key(),
        authority: ctx.accounts.authority.key(),
        is_active: pool.is_active,
    });

    Ok(())
}

pub fn close_pool_handler(ctx: Context<ClosePool>) -> Result<()> {
    let pool = &ctx.accounts.pool;

    check_authority(&ctx.accounts.authority.key(), &pool.authority)?;

    emit!(PoolClosed {
        pool: pool.key(),
        authority: ctx.accounts.authority.key(),
        final_sol_reserve: pool.sol_reserve,
        final_token_reserve: pool.token_reserve,
    });

    Ok(())
}