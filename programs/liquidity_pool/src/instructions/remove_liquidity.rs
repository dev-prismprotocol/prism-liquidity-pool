// programs/liquidity_pool/src/instructions/remove_liquidity.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, TransferChecked, transfer_checked};
use crate::state::*;
use crate::error::*;
use crate::events::*;

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump,
        constraint = pool.authority == user.key() @ LiquidityPoolError::UnauthorizedAccess
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = user_token_account.mint == pool.token_mint,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_token", pool.token_mint.as_ref()],
        bump
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn remove_liquidity_handler(
    ctx: Context<RemoveLiquidity>,
    token_decimals: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let user = &ctx.accounts.user;

    require!(token_decimals <= 18, LiquidityPoolError::InvalidSwapParams);

    let sol_to_withdraw = pool.sol_reserve;
    let pool_token_balance = ctx.accounts.pool_token_account.amount;

    require!(sol_to_withdraw > 0 || pool_token_balance > 0, LiquidityPoolError::InsufficientLiquidity);

    if sol_to_withdraw > 0 {
        require!(
            pool.to_account_info().lamports() >= sol_to_withdraw,
            LiquidityPoolError::InsufficientLiquidity
        );

        **pool.to_account_info().try_borrow_mut_lamports()? -= sol_to_withdraw;
        **user.to_account_info().try_borrow_mut_lamports()? += sol_to_withdraw;
    }

    if pool_token_balance > 0 {
        let seeds = &[b"pool", pool.token_mint.as_ref(), &[pool.bump]];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.pool_token_account.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    mint: ctx.accounts.token_mint.to_account_info(),
                    authority: pool.to_account_info(),
                },
                &[seeds]
            ),
            pool_token_balance,
            token_decimals,
        )?;
    }

    emit!(LiquidityRemoved {
        pool: pool.key(),
        liquidity_provider: user.key(),
        sol_amount: sol_to_withdraw,
        token_amount: pool_token_balance,
        dex_name: DEX_NAME.to_string(),
        transaction_type: "remove_liquidity".to_string(),
    });

    pool.sol_reserve = 0;
    pool.token_reserve = 0;
    pool.is_active = false;

    Ok(())
}