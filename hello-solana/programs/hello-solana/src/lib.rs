use anchor_lang::prelude::*;

declare_id!("4d7Grr2mQuPcVoASrf3k91LuRGk21LVFbsjWL1L5p3W5");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct Initialize {}
