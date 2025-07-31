// programs/liquidity_pool/src/error.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum LiquidityPoolError {
    #[msg("Unauthorized access")]
    UnauthorizedAccess,

    #[msg("Pool is currently inactive")]
    PoolInactive,

    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity,

    #[msg("Invalid exchange rate")]
    InvalidExchangeRate,

    #[msg("Invalid swap amount")]
    InvalidSwapAmount,

    #[msg("Mathematical overflow")]
    MathOverflow,

    #[msg("Fee exceeds maximum allowed")]
    FeeExceedsMaximum,

    #[msg("Pool reserves are invalid")]
    InvalidReserves,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Pool already exists for this token")]
    PoolAlreadyExists,

    #[msg("Invalid swap parameters")]
    InvalidSwapParams,
}