// programs/liquidity_pool/src/lib.rs
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;
pub mod events;
pub mod utils;

use instructions::*;

declare_id!("G7G2o7vaMvacSkMENdBjqjECjpK3USBpDmSzpDo6JWrf");

#[program]
pub mod liquidity_pool {
    use super::*;

    pub fn create_pool(
        ctx: Context<CreatePool>,
        virtual_token_reserve: Option<u64>,
        virtual_sol_reserve: Option<u64>,
    ) -> Result<()> {
        instructions::create_pool::handler(ctx, virtual_token_reserve, virtual_sol_reserve)
    }

    pub fn create_pool_token_account(ctx: Context<CreatePoolTokenAccount>) -> Result<()> {
        instructions::create_pool_token_account::create_pool_token_account_handler(ctx)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        sol_amount: u64,
        token_amount: u64,
        token_decimals: u8,
    ) -> Result<()> {
        instructions::add_liquidity::add_liquidity_handler(ctx, sol_amount, token_amount, token_decimals)
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        token_decimals: u8,
    ) -> Result<()> {
        instructions::remove_liquidity::remove_liquidity_handler(ctx, token_decimals)
    }

    pub fn swap_tokens_for_sol(
        ctx: Context<SwapTokensForSol>,
        token_amount: u64,
        sol_output: u64,
        token_decimals: u8,
    ) -> Result<()> {
        instructions::swap::swap_tokens_for_sol_handler(ctx, token_amount, sol_output, token_decimals)
    }

    pub fn swap_sol_for_tokens(
        ctx: Context<SwapSolForTokens>,
        sol_amount: u64,
        token_output: u64,
        token_decimals: u8,
    ) -> Result<()> {
        instructions::swap::swap_sol_for_tokens_handler(ctx, sol_amount, token_output, token_decimals)
    }

    pub fn update_exchange_rate(
        ctx: Context<UpdateExchangeRate>,
        new_rate: u64,
    ) -> Result<()> {
        instructions::admin::update_exchange_rate_handler(ctx, new_rate)
    }

    pub fn toggle_pool(ctx: Context<TogglePool>) -> Result<()> {
        instructions::admin::toggle_pool_handler(ctx)
    }

    pub fn close_pool(ctx: Context<ClosePool>) -> Result<()> {
        instructions::admin::close_pool_handler(ctx)
    }

    pub fn get_pool_info(ctx: Context<GetPoolInfo>) -> Result<()> {
        instructions::query::get_pool_info_handler(ctx)
    }

    pub fn calculate_swap(
        ctx: Context<CalculateSwap>,
        input_amount: u64,
        token_to_sol: bool,
    ) -> Result<()> {
        instructions::query::calculate_swap_handler(ctx, input_amount, token_to_sol)
    }
}
