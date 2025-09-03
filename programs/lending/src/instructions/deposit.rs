use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::jupiter_dev;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // User accounts
    pub signer: Signer<'info>,
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    // Protocol accounts
    pub lending_admin: UncheckedAccount<'info>,
    pub lending: UncheckedAccount<'info>,
    pub f_token_mint: UncheckedAccount<'info>,

    // Liquidity protocol accounts
    pub supply_token_reserves_liquidity: UncheckedAccount<'info>,
    pub lending_supply_position_on_liquidity: UncheckedAccount<'info>,
    pub rate_model: UncheckedAccount<'info>,
    pub vault: UncheckedAccount<'info>,
    pub liquidity: UncheckedAccount<'info>,
    pub liquidity_program: UncheckedAccount<'info>,

    // Rewards and programs
    pub rewards_rate_model: UncheckedAccount<'info>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,

    // Target lending program
    pub lending_program: Program<'info, jupiter_dev::program::Lending>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&self, amount: u64) -> Result<()> {
        msg!("Depositing {} lamports", amount);

        let cpi_accounts = jupiter_dev::cpi::accounts::Deposit {
            associated_token_program: self.system_program.to_account_info(),
            depositor_token_account: self.signer.to_account_info(),
            f_token_mint: self.system_program.to_account_info(),
            lending: self.system_program.to_account_info(),
            lending_admin: self.system_program.to_account_info(),
            lending_supply_position_on_liquidity: self.system_program.to_account_info(),
            liquidity: self.system_program.to_account_info(),
            liquidity_program: self.system_program.to_account_info(),
            mint: self.system_program.to_account_info(),
            rate_model: self.system_program.to_account_info(),
            recipient_token_account: self.system_program.to_account_info(),
            rewards_rate_model: self.system_program.to_account_info(),
            signer: self.system_program.to_account_info(),
            supply_token_reserves_liquidity: self.system_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.system_program.to_account_info(),
            vault: self.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        let res = jupiter_dev::cpi::deposit(cpi_ctx, amount)?;
        msg!("Deposit result: {}", res.get());

        Ok(())
    }
}
