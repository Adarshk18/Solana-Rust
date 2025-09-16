use anchor_lang::prelude::*;

declare_id!("4d7Grr2mQuPcVoASrf3k91LuRGk21LVFbsjWL1L5p3W5");

#[program]
// // pub mod hello_solana {
// //     use super::*;

// //     pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
// //         ctx.accounts.new_account.data = data;
// //         msg!("Changed data to: {}!", data);
// //         Ok(())
// //     }
// // }

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     #[account(
//         init,
//         payer=signer,
//         space = 8+8
//     )]
//     pub new_account: Account<'info, NewAccount>,
//     #[account(mut)]
//     pub signer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[account]
// pub struct NewAccount {
//     data: u64,
// }


pub mod pda{
    use super::*;

    pub fn create(_ctx: Context<Create>) -> Result<()>{
        Ok(())
    }

    pub fn update(_ctx: Context<Update>)->Result<()>{
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>)-> Result<()>{
        Ok(())

    }
}


#[derive(Accounts)]
#[instruction(message: String)]
pub struct Create<'info>{
    #[account(mut)]
    pub user: Signer<'info>, //represents the user creating message account
    #[account(
        init,
        seeds = [b"message",user.key().as_ref()],
        bump,
        payer = user,
        space = 8+32+4+message.len()+1
    )]

    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update {}

#[derive(Accounts)]
pub struct Delete {}

#[account]
pub struct MessageAccount {
    pub user: Pubkey,
    pub message: String,
    pub bump: u8,
}
