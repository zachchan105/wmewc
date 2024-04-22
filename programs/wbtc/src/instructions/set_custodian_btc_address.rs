use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::{Config, Merchant};
use crate::utils::validate_mewc_address;

#[derive(Accounts)]
pub struct SetCustodianMewcAddressAccounts<'info> {
    #[account(mut)]
    pub custodian: Signer<'info>,

    #[account(mut, has_one = custodian @ ErrorCode::InvalidCustodian)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub merchant: Account<'info, Merchant>,
}

pub fn handler(
    ctx: Context<SetCustodianMewcAddressAccounts>,
    new_custodian_mewc_address: String,
) -> Result<()> {
    require!(
        ctx.accounts.config.custodian_enabled,
        ErrorCode::CustodianDisabled
    );

    validate_mewc_address(&new_custodian_mewc_address)?;

    ctx.accounts.merchant.custodian_mewc_address = new_custodian_mewc_address;

    Ok(())
}
