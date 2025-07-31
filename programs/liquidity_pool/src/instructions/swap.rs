// programs/liquidity_pool/src/instructions/swap.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, TransferChecked, transfer_checked};
use crate::state::*;
use crate::error::*;
use crate::events::*;
use crate::utils::math::*;

#[derive(Accounts)]
pub struct SwapTokensForSol<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        constraint = program_authority.key() == crate::ID @ LiquidityPoolError::UnauthorizedAccess
    )]
    pub program_authority: Signer<'info>,

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

    #[account(
        seeds = [b"__event_authority"],
        bump
    )]
    pub event_authority: UncheckedAccount<'info>,

    pub program: Program<'info, crate::program::LiquidityPool>,
}

#[derive(Accounts)]
pub struct SwapSolForTokens<'info> {
    #[account(
        mut,
        seeds = [b"pool", pool.token_mint.as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LiquidityPool>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        constraint = program_authority.key() == crate::ID @ LiquidityPoolError::UnauthorizedAccess
    )]
    pub program_authority: Signer<'info>,

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

    #[account(
        seeds = [b"__event_authority"],
        bump
    )]
    pub event_authority: UncheckedAccount<'info>,

    pub program: Program<'info, crate::program::LiquidityPool>,
}

pub fn swap_tokens_for_sol_handler(
    ctx: Context<SwapTokensForSol>,
    token_amount: u64,
    sol_output: u64,
    token_decimals: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let current_time = Clock::get()?.unix_timestamp;

    require!(pool.is_active, LiquidityPoolError::PoolInactive);
    require!(token_decimals <= 18, LiquidityPoolError::InvalidSwapParams);

    const AUTO_VALUE: u64 = u64::MAX;

    let (final_token_amount, final_sol_output) = match (token_amount == AUTO_VALUE, sol_output == AUTO_VALUE) {
        (false, true) => {
            require!(token_amount > 0, LiquidityPoolError::InvalidSwapAmount);

            let normalized_token_input = normalize_token_amount(token_amount, token_decimals)?;

            let rent = Rent::get()?;
            let rent_exempt_minimum = rent.minimum_balance(std::mem::size_of::<LiquidityPool>());

            let (real_sol, real_tokens) = get_real_reserves_from_accounts(
                pool.to_account_info().lamports(),
                rent_exempt_minimum,
                ctx.accounts.pool_token_account.amount,
                token_decimals,
            )?;

            let (effective_sol_reserve, effective_token_reserve) = get_effective_pool_reserves(
                pool.sol_reserve,
                pool.token_reserve,
                real_sol,
                real_tokens,
            )?;

            require!(effective_sol_reserve > 0 && effective_token_reserve > 0, LiquidityPoolError::InsufficientLiquidity);

            let calculated_sol_output = calculate_swap_output_amount(
                normalized_token_input,
                effective_token_reserve,
                effective_sol_reserve,
            )?;

            (token_amount, calculated_sol_output)
        },
        _ => {
            return Err(LiquidityPoolError::InvalidSwapParams.into());
        }
    };

    require!(final_sol_output > 0, LiquidityPoolError::InvalidSwapAmount);

    let current_account_balance = pool.to_account_info().lamports();
    let rent = Rent::get()?;
    let rent_exempt_minimum = rent.minimum_balance(std::mem::size_of::<LiquidityPool>());

    require!(
        current_account_balance >= rent_exempt_minimum.checked_add(final_sol_output)
            .ok_or(LiquidityPoolError::MathOverflow)?,
        LiquidityPoolError::InsufficientLiquidity
    );

    emit!(SwapExecuted {
        pool: pool.key(),
        trader: ctx.accounts.user.key(),
        token_mint: pool.token_mint,
        dex_name: DEX_NAME.to_string(),
        trade_direction: "sell".to_string(),
        token_amount: final_token_amount,
        sol_amount: final_sol_output,
        exchange_rate: if final_token_amount > 0 { (final_sol_output as f64) / (final_token_amount as f64) } else { 0.0 },
        timestamp: current_time,
        transaction_type: TRANSACTION_TYPE.to_string(),
        pool_type: POOL_TYPE.to_string(),
        slippage: 0.0,
        fee_amount: 0,
        program_version: PROGRAM_VERSION.to_string(),
        network: get_network_name(),
    });

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                mint: ctx.accounts.token_mint.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        final_token_amount,
        token_decimals,
    )?;

    **pool.to_account_info().try_borrow_mut_lamports()? -= final_sol_output;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += final_sol_output;

    let normalized_final_token_amount = normalize_token_amount(final_token_amount, token_decimals)?;
    pool.token_reserve = pool.token_reserve.checked_add(normalized_final_token_amount)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    pool.update_trade_stats(final_sol_output, final_token_amount, current_time);

    Ok(())
}

pub fn swap_sol_for_tokens_handler(
    ctx: Context<SwapSolForTokens>,
    sol_amount: u64,
    token_output: u64,
    token_decimals: u8,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let current_time = Clock::get()?.unix_timestamp;

    require!(pool.is_active, LiquidityPoolError::PoolInactive);
    require!(token_decimals <= 18, LiquidityPoolError::InvalidSwapParams);

    const AUTO_VALUE: u64 = u64::MAX;

    let (final_sol_amount, final_token_output) = match (sol_amount == AUTO_VALUE, token_output == AUTO_VALUE) {
        (true, false) => {
            require!(token_output > 0, LiquidityPoolError::InvalidSwapAmount);

            let normalized_token_output = normalize_token_amount(token_output, token_decimals)?;

            let rent = Rent::get()?;
            let rent_exempt_minimum = rent.minimum_balance(std::mem::size_of::<LiquidityPool>());

            let (real_sol, real_tokens) = get_real_reserves_from_accounts(
                pool.to_account_info().lamports(),
                rent_exempt_minimum,
                ctx.accounts.pool_token_account.amount,
                token_decimals,
            )?;

            let (effective_sol_reserve, effective_token_reserve) = get_effective_pool_reserves(
                pool.sol_reserve,
                pool.token_reserve,
                real_sol,
                real_tokens,
            )?;

            require!(effective_sol_reserve > 0 && effective_token_reserve > 0, LiquidityPoolError::InsufficientLiquidity);

            let calculated_sol_amount = calculate_swap_input_amount(
                normalized_token_output,
                effective_sol_reserve,
                effective_token_reserve,
            )?;

            (calculated_sol_amount, token_output)
        },
        _ => {
            return Err(LiquidityPoolError::InvalidSwapParams.into());
        }
    };

    require!(final_token_output > 0, LiquidityPoolError::InvalidSwapAmount);
    require!(final_token_output <= ctx.accounts.pool_token_account.amount, LiquidityPoolError::InsufficientLiquidity);
    require!(final_sol_amount > 0, LiquidityPoolError::InvalidSwapAmount);

    let rent = Rent::get()?;
    let rent_exempt_minimum = rent.minimum_balance(std::mem::size_of::<LiquidityPool>());
    let current_account_balance = pool.to_account_info().lamports();
    require!(
        current_account_balance >= rent_exempt_minimum.checked_add(final_sol_amount)
            .ok_or(LiquidityPoolError::MathOverflow)?,
        LiquidityPoolError::InsufficientLiquidity
    );

    emit!(SwapExecuted {
        pool: pool.key(),
        trader: ctx.accounts.user.key(),
        token_mint: pool.token_mint,
        dex_name: DEX_NAME.to_string(),
        trade_direction: "buy".to_string(),
        token_amount: final_token_output,
        sol_amount: final_sol_amount,
        exchange_rate: if final_token_output > 0 { (final_sol_amount as f64) / (final_token_output as f64) } else { 0.0 },
        timestamp: current_time,
        transaction_type: TRANSACTION_TYPE.to_string(),
        pool_type: POOL_TYPE.to_string(),
        slippage: 0.0,
        fee_amount: 0,
        program_version: PROGRAM_VERSION.to_string(),
        network: get_network_name(),
    });

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: pool.to_account_info(),
            },
        ),
        final_sol_amount,
    )?;

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
        final_token_output,
        token_decimals,
    )?;

    let normalized_final_token_output = normalize_token_amount(final_token_output, token_decimals)?;
    pool.token_reserve = pool.token_reserve.checked_sub(normalized_final_token_output)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    pool.update_trade_stats(final_sol_amount, final_token_output, current_time);

    Ok(())
}