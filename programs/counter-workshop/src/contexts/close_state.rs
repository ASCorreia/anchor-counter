use anchor_lang::prelude::*;

use crate::state::CounterPDA;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"counter", user.key().as_ref()],
        bump = counter_pda.bump,
    )]
    counter_pda: Account<'info, CounterPDA>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        msg!("Account Closed!");

        Ok(())
    }
}