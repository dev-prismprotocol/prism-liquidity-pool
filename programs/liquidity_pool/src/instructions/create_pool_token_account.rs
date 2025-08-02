// programs/liquidity_pool/src/instructions/create_pool_token_account.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::*;

#[derive(Accounts)]
pub struct CreatePoolTokenAccount<'info> {
    #[account(
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        seeds = [b"pool_token", token_mint.key().as_ref()],
        bump,
        token::mint = token_mint,
        token::authority = pool
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_pool_token_account_handler(_ctx: Context<CreatePoolTokenAccount>) -> Result<()> {
    Ok(())
}