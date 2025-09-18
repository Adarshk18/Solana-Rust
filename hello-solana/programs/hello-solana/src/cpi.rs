use anchor_program::system_program::{transfer,Transfer};

pub mod cpi{
    use super::*;

     pub fn update(ctx: Context<Update>, message: String)->Result<()>{
        msg!("Update Message: {}", message);
        let account_data = &mut ctx.accounts.message_account;
        account_data.message = message;

        let transfer_account = Transfer{
            from: ctx.account.user.to_account_info(),
            to: ctx.account.vault_account.to_account_info()
        };

        let cpi_context = CpiContext::new(
            ctx.account.system_program.to_account_info(),
            transfer_account
        );

        transfer(cpi_context, 1_000_000)?;
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>, message: String)-> Result<()>{
        msg!("Delete Message");
        Ok(())

    }
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault",user.key().as_ref()],
        bump,
    )]
    pub vault_account: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"message",user.key().as_ref()],
        bump = message_account.bump,
        realloc = 8+32+4+message.len()+1,
        realloc::payer = user,
        realloc::zero = true,
    )]

    pub message_account: Account<'info, MessageAccount>,
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"message", user.key().as_ref()],
        bump = message_account.bump,
        close = user,
    )]
    pub message_account: Account<'info, MessageAccount>,
}