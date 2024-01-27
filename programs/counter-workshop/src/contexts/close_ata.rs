use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken,token::{
    burn, close_account, Burn, CloseAccount, Mint, Token, TokenAccount
}};
use anchor_spl::metadata::ID as METADATA_PROGRAM_ID;

use crate::state::CounterPDA;

#[derive(Accounts)]
pub struct CloseATA<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        mint::decimals = 6,
        mint::authority = counter_pda,
    )]
    mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_ata: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous and will be checked by the metaplex program
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    counter_pda: Account<'info, CounterPDA>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = METADATA_PROGRAM_ID)]
    token_metadata_program: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

impl<'info> CloseATA<'info> {
    pub fn close_ata(&mut self) -> Result<()> {
        let seeds = &[
            &b"counter"[..],
            &self.user.key().to_bytes()[..], 
            &[self.counter_pda.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            from: self.user_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        burn(cpi_ctx, 1_000_000)?;

        msg!("Burned tokens");

        let cpi_accounts = CloseAccount {
            account: self.user_ata.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_program = self.associated_token_program.to_account_info();

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;
        
        Ok(())
    }
}