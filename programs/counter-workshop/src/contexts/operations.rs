use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

impl<'info> Operations<'info> {
    pub fn increment(&mut self) -> Result<()> {
        self.counter.counter += 1;

        Ok(())
    }

    pub fn decrement(&mut self) -> Result<()> {
        match self.counter.counter {
            1..=u8::MAX => self.counter.counter -= 1,
            _ => (),
        }
        
        Ok(())
    }
}