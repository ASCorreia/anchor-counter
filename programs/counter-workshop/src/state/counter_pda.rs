use anchor_lang::prelude::*;

#[account]
pub struct CounterPDA {
    pub counter: u8,
    pub bump: u8,
}

impl Space for CounterPDA {
    const INIT_SPACE: usize = 8 + 1 +1;
}
