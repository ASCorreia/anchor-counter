use anchor_lang::prelude::*;

declare_id!("6ubN8kmjebNWoQFU2x8qn941gHyQ5Lp3UFV7WCHfDXKu");

pub mod state;
pub mod contexts;

pub use contexts::*;

#[program]
pub mod counter_workshop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()
    }

    pub fn initialize_pda(ctx: Context<InitPDA>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn increment(ctx: Context<Operations>) -> Result<()> {
        ctx.accounts.increment()
    }

    pub fn increment_pda(ctx: Context<OperationsPDA>) -> Result<()> {
        ctx.accounts.increment()
    }

    pub fn decrement(ctx: Context<Operations>) -> Result<()> {
        ctx.accounts.decrement()
    }

    pub fn decrement_pda(ctx: Context<OperationsPDA>) -> Result<()> {
        ctx.accounts.decrement()
    }

    pub fn mint_spl(ctx: Context<MintSPL>) -> Result<()> {
        ctx.accounts.mint_spl()?;
        ctx.accounts.associate_metadata()
    }
}
