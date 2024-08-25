use anchor_lang::prelude::*;

declare_id!("8JkjEncS1HrJFmSDo1PTmbtLHacQGc2mMBfEe54TXxGS");

#[program]
pub mod sol_pension {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
