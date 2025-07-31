// programs/liquidity_pool/src/utils/math.rs
use anchor_lang::prelude::*;
use crate::error::LiquidityPoolError;

pub fn calculate_tokens_to_sol(token_amount: u64, exchange_rate: u64) -> Result<u64> {
    if exchange_rate == 0 {
        return Err(LiquidityPoolError::MathOverflow.into());
    }

    if token_amount <= u64::MAX / 1_000_000 {
        let temp = token_amount
            .checked_mul(1_000_000)
            .ok_or(LiquidityPoolError::MathOverflow)?;
        temp.checked_div(exchange_rate)
            .ok_or(LiquidityPoolError::MathOverflow.into())
    } else {
        let temp = token_amount
            .checked_div(exchange_rate)
            .ok_or(LiquidityPoolError::MathOverflow)?;
        temp.checked_mul(1_000_000)
            .ok_or(LiquidityPoolError::MathOverflow.into())
    }
}

pub fn calculate_sol_to_tokens(sol_amount: u64, exchange_rate: u64) -> Result<u64> {
    if exchange_rate == 0 {
        return Err(LiquidityPoolError::MathOverflow.into());
    }

    if exchange_rate <= u64::MAX / sol_amount {
        let temp = sol_amount
            .checked_mul(exchange_rate)
            .ok_or(LiquidityPoolError::MathOverflow)?;
        temp.checked_div(1_000_000)
            .ok_or(LiquidityPoolError::MathOverflow.into())
    } else {
        let temp = sol_amount
            .checked_div(1_000_000)
            .unwrap_or(0);
        temp.checked_mul(exchange_rate)
            .ok_or(LiquidityPoolError::MathOverflow.into())
    }
}

pub fn calculate_fee(amount: u64, fee_basis_points: u16) -> Result<u64> {
    if fee_basis_points == 0 {
        return Ok(0);
    }

    amount
        .checked_mul(fee_basis_points as u64)
        .and_then(|x| x.checked_div(10000))
        .ok_or(LiquidityPoolError::MathOverflow.into())
}

pub fn safe_add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b).ok_or(LiquidityPoolError::MathOverflow.into())
}

pub fn safe_sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b).ok_or(LiquidityPoolError::MathOverflow.into())
}

pub fn safe_mul(a: u64, b: u64) -> Result<u64> {
    a.checked_mul(b).ok_or(LiquidityPoolError::MathOverflow.into())
}

pub fn safe_div(a: u64, b: u64) -> Result<u64> {
    if b == 0 {
        return Err(LiquidityPoolError::MathOverflow.into());
    }
    Ok(a / b)
}

pub fn safe_mul_div(a: u64, b: u64, c: u64) -> Result<u64> {
    if c == 0 {
        return Err(LiquidityPoolError::MathOverflow.into());
    }

    let a_u128 = a as u128;
    let b_u128 = b as u128;
    let c_u128 = c as u128;

    let result = a_u128
        .checked_mul(b_u128)
        .ok_or(LiquidityPoolError::MathOverflow)?
        .checked_div(c_u128)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    if result > u64::MAX as u128 {
        return Err(LiquidityPoolError::MathOverflow.into());
    }

    Ok(result as u64)
}

pub fn normalize_token_amount(amount: u64, token_decimals: u8) -> Result<u64> {
    const SOL_DECIMALS: u8 = 9;

    if token_decimals == SOL_DECIMALS {
        return Ok(amount);
    }

    if token_decimals > SOL_DECIMALS {
        let scale_down = 10_u64.pow((token_decimals - SOL_DECIMALS) as u32);
        Ok(amount.checked_div(scale_down).unwrap_or(0))
    } else {
        let scale_up = 10_u64.pow((SOL_DECIMALS - token_decimals) as u32);
        amount.checked_mul(scale_up).ok_or(LiquidityPoolError::MathOverflow.into())
    }
}

pub fn get_effective_pool_reserves(
    virtual_sol_reserve: u64,
    virtual_token_reserve: u64,
    real_sol_balance: u64,
    real_token_balance: u64,
) -> Result<(u64, u64)> {
    let effective_sol = virtual_sol_reserve
        .checked_add(real_sol_balance)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    let effective_token = virtual_token_reserve
        .checked_add(real_token_balance)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    Ok((effective_sol, effective_token))
}

pub fn calculate_swap_output_amount(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
) -> Result<u64> {
    if input_reserve == 0 || output_reserve == 0 {
        return Err(LiquidityPoolError::InsufficientLiquidity.into());
    }

    let new_input_reserve = input_reserve
        .checked_add(input_amount)
        .ok_or(LiquidityPoolError::MathOverflow)?;

    safe_mul_div(output_reserve, input_amount, new_input_reserve)
}

pub fn calculate_swap_input_amount(
    output_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
) -> Result<u64> {
    if input_reserve == 0 || output_reserve == 0 {
        return Err(LiquidityPoolError::InsufficientLiquidity.into());
    }

    if output_amount >= output_reserve {
        return Err(LiquidityPoolError::InsufficientLiquidity.into());
    }

    let new_output_reserve = output_reserve
        .checked_sub(output_amount)
        .ok_or(LiquidityPoolError::InsufficientLiquidity)?;

    safe_mul_div(input_reserve, output_amount, new_output_reserve)
}

pub fn get_real_reserves_from_accounts(
    pool_lamports: u64,
    rent_exempt_minimum: u64,
    pool_token_amount: u64,
    token_decimals: u8,
) -> Result<(u64, u64)> {
    let real_sol = pool_lamports
        .checked_sub(rent_exempt_minimum)
        .unwrap_or(0);

    let real_tokens = normalize_token_amount(pool_token_amount, token_decimals)?;

    Ok((real_sol, real_tokens))
}