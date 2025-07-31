// programs/liquidity_pool/src/utils/auth.rs
use anchor_lang::prelude::*;
use crate::error::LiquidityPoolError;

pub fn check_authority(user: &Pubkey, authority: &Pubkey) -> Result<()> {
    require!(user == authority, LiquidityPoolError::UnauthorizedAccess);
    Ok(())
}