use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::jupiter_dev::{self};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // User accounts
    pub signer: Signer<'info>,
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    // Protocol accounts
    pub lending_admin: Account<'info, jupiter_dev::accounts::LendingAdmin>,
    pub lending: Account<'info, jupiter_dev::accounts::Lending>,
    pub f_token_mint: InterfaceAccount<'info, Mint>,

    // Liquidity protocol accounts
    /// CHECK: verify by jupiter
    pub supply_token_reserves_liquidity: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub lending_supply_position_on_liquidity: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub rate_model: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub vault: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub claim_account: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub liquidity: UncheckedAccount<'info>,
    /// CHECK: verify by jupiter
    pub liquidity_program: UncheckedAccount<'info>,

    // Rewards and programs
    pub rewards_rate_model: Account<'info, jupiter_dev::accounts::LendingRewardsRateModel>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    // Target lending program
    pub lending_program: Program<'info, jupiter_dev::program::Lending>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, amount: u64) -> Result<()> {
        msg!("Withdrawing {} lamports", amount);

        let cpi_accounts = jupiter_dev::cpi::accounts::Withdraw {
            associated_token_program: self.associated_token_program.to_account_info(),
            claim_account: self.claim_account.to_account_info(),
            signer: self.signer.to_account_info(),
            f_token_mint: self.f_token_mint.to_account_info(),
            lending: self.lending.to_account_info(),
            lending_admin: self.lending_admin.to_account_info(),
            lending_supply_position_on_liquidity: self
                .lending_supply_position_on_liquidity
                .to_account_info(),
            liquidity: self.liquidity.to_account_info(),
            liquidity_program: self.liquidity_program.to_account_info(),
            supply_token_reserves_liquidity: self.supply_token_reserves_liquidity.to_account_info(),
            mint: self.mint.to_account_info(),
            owner_token_account: self.owner_token_account.to_account_info(),
            rate_model: self.rate_model.to_account_info(),
            rewards_rate_model: self.rewards_rate_model.to_account_info(),
            recipient_token_account: self.recipient_token_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
            vault: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.lending_program.to_account_info(), cpi_accounts);

        let res = jupiter_dev::cpi::withdraw(cpi_ctx, amount)?;
        msg!("Withdraw result: {}", res.get());

        Ok(())
    }
}
