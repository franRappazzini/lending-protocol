use anchor_lang::prelude::*;

use crate::jupiter_dev::{self};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&self, amount: u64) -> Result<()> {
        msg!("Withdrawing {} lamports", amount);

        let cpi_accounts = jupiter_dev::cpi::accounts::Withdraw {
            associated_token_program: self.system_program.to_account_info(),
            claim_account: self.system_program.to_account_info(),
            signer: self.system_program.to_account_info(),
            f_token_mint: self.system_program.to_account_info(),
            lending: self.system_program.to_account_info(),
            lending_admin: self.system_program.to_account_info(),
            lending_supply_position_on_liquidity: self.system_program.to_account_info(),
            liquidity: self.system_program.to_account_info(),
            liquidity_program: self.system_program.to_account_info(),
            supply_token_reserves_liquidity: self.system_program.to_account_info(),
            mint: self.system_program.to_account_info(),
            owner_token_account: self.system_program.to_account_info(),
            rate_model: self.system_program.to_account_info(),
            rewards_rate_model: self.system_program.to_account_info(),
            recipient_token_account: self.system_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.system_program.to_account_info(),
            vault: self.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        let res = jupiter_dev::cpi::withdraw(cpi_ctx, amount)?;
        msg!("Withdraw result: {}", res.get());

        Ok(())
    }
}
