use anchor_lang::prelude::*;

pub mod contexts;
pub mod errors;
pub mod state;

use contexts::*;

declare_id!("3bUMiDBPPenwzWm8uCQZBchTGckRNCEUBC2tzd1aFBxx");

#[program]
pub mod sol_trust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, lock_duration: i64) -> Result<()> {
        ctx.accounts.initialize(lock_duration, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn premature_close(ctx: Context<PrematureClose>) -> Result<()> {
        ctx.accounts.premature_close()?;
        Ok(())
    }

    // pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    //     ctx.accounts.cancel()?;
    //     Ok(())
    // }
}
