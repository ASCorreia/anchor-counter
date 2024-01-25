use anchor_lang::prelude::*;

use crate::state::CounterPDA;

#[derive(Accounts)]
pub struct OperationsPDA<'info> {
    user: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()],
        bump = counter_pda.bump,
    )]
    counter_pda: Account<'info, CounterPDA>,
}

impl<'info> OperationsPDA<'info> {
    pub fn increment(&mut self) -> Result<()> {
        self.counter_pda.counter += 1;

        Ok(())
    }

    pub fn decrement(&mut self) -> Result<()> {
        match self.counter_pda.counter {
            1..=u8::MAX => self.counter_pda.counter -= 1,
            _ => (),
        }
        
        Ok(())
    }
}