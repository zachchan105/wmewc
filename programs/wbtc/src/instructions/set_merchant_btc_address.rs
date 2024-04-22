use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::Merchant;
use crate::utils::validate_mewc_address;

#[derive(Accounts)]
pub struct SetMerchantMewcAddressAccounts<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority @ ErrorCode::InvalidMerchantAuthority)]
    pub merchant: Account<'info, Merchant>,
}

pub fn handler(
    ctx: Context<SetMerchantMewcAddressAccounts>,
    new_merchant_mewc_address: String,
) -> Result<()> {
    validate_mewc_address(&new_merchant_mewc_address)?;

    ctx.accounts.merchant.mewc_address = new_merchant_mewc_address;

    Ok(())
}
