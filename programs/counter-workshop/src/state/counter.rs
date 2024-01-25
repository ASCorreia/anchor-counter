use anchor_lang::prelude::*;

#[account]
pub struct Counter {
    pub counter: u8,
}

impl Space for Counter {
    const INIT_SPACE: usize = 8 + 1;
}