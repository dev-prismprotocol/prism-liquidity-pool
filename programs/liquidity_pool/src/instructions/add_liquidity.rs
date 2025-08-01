// programs/liquidity_pool/src/instructions/add_liquidity.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, TransferChecked, transfer_checked};
use crate::state::*;
use crate::error::*;
use crate::events::*;
use crate::utils::math::*;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = authority_token_account.mint == token_mint.key(),
        constraint = authority_token_account.owner == authority.key()
    )]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_token", token_mint.key().as_ref()],
        bump
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn add_liquidity_handler(
    ctx: Context<AddLiquidity>,
    sol_amount: u64,
    token_amount: u64,
    token_decimals: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let authority = &ctx.accounts.authority;

    require!(sol_amount > 0 || token_amount > 0, LiquidityPoolError::InvalidReserves);
    require!(token_decimals <= 18, LiquidityPoolError::InvalidSwapParams);

    let normalized_token_amount = if token_amount > 0 {
        normalize_token_amount(token_amount, token_decimals)?
    } else {
        0
    };

    if token_amount > 0 {
        require!(
            ctx.accounts.authority_token_account.amount >= token_amount,
            LiquidityPoolError::InsufficientLiquidity
        );

        let transfer_tokens_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.authority_token_account.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                authority: authority.to_account_info(),
            },
        );
        transfer_checked(transfer_tokens_ctx, token_amount, token_decimals)?;
    }

    if sol_amount > 0 {
        require!(
            authority.lamports() >= sol_amount,
            LiquidityPoolError::InsufficientLiquidity
        );

        let transfer_sol_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: authority.to_account_info(),
                to: pool.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_sol_ctx, sol_amount)?;
    }

    if sol_amount > 0 {
        require!(sol_amount <= pool.sol_reserve, LiquidityPoolError::InsufficientLiquidity);
        pool.sol_reserve = pool.sol_reserve.checked_sub(sol_amount)
            .ok_or(LiquidityPoolError::MathOverflow)?;
    }

    if normalized_token_amount > 0 {
        require!(normalized_token_amount <= pool.token_reserve, LiquidityPoolError::InsufficientLiquidity);
        pool.token_reserve = pool.token_reserve.checked_sub(normalized_token_amount)
            .ok_or(LiquidityPoolError::MathOverflow)?;
    }

    pool.exchange_rate = if pool.sol_reserve > 0 && pool.token_reserve > 0 {
        if pool.token_reserve <= u64::MAX / 1_000_000 {
            pool.token_reserve.checked_mul(1_000_000)
                .ok_or(LiquidityPoolError::MathOverflow)?
                .checked_div(pool.sol_reserve)
                .unwrap_or(1)
        } else {
            pool.token_reserve.checked_div(pool.sol_reserve)
                .ok_or(LiquidityPoolError::MathOverflow)?
                .checked_mul(1_000_000)
                .unwrap_or(1)
        }
    } else {
        1
    };

    pool.is_active = true;

    emit!(LiquidityAdded {
        pool: pool.key(),
        liquidity_provider: authority.key(),
        sol_amount,
        token_amount,
        dex_name: DEX_NAME.to_string(),
        transaction_type: "add_liquidity".to_string(),
    });

    Ok(())
}
