mod instructions;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("JDWKbtsL4hrmhL2eL56Mfxd2qJqBngpsnd5hQzgnSV4b");

declare_program!(jupiter_dev);

#[program]
pub mod lending {
    use crate::instructions::Deposit;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
