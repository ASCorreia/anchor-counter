use anchor_lang::prelude::*;

use crate::state::CounterPDA;

#[derive(Accounts)]
pub struct InitPDA<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init,
        seeds = [b"counter", user.key().as_ref()],
        bump,
        payer = user,
        space = CounterPDA::INIT_SPACE,
    )]
    pub counter_pda: Account<'info, CounterPDA>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitPDA<'info> {
    pub fn initialize(&mut self, bumps: &InitPDABumps) -> Result<()> {
        self.counter_pda.counter = 0;
        self.counter_pda.bump = bumps.counter_pda;

        Ok(())
    }
}