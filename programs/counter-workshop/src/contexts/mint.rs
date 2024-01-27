use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken,token::{
    close_account, mint_to, CloseAccount, Mint, MintTo, Token, TokenAccount
}};
use mpl_token_metadata::{instructions::{CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs}, types::{Creator, DataV2}};
use anchor_spl::metadata::ID as METADATA_PROGRAM_ID;

use crate::state::CounterPDA;

#[derive(Accounts)]
pub struct MintSPL<'info> {
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
        init_if_needed,
        payer = user,
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

impl<'info> MintSPL<'info> {
    pub fn mint_spl(&mut self) -> Result<()> {
        match self.counter_pda.counter {
            1 => {
                let seeds = &[
                    &b"counter"[..],
                    &self.user.key().to_bytes()[..], 
                    &[self.counter_pda.bump]
                ];
                let signer_seeds = &[&seeds[..]];

                let cpi_accounts = MintTo {
                    mint: self.mint.to_account_info(),
                    to: self.user_ata.to_account_info(),
                    authority: self.counter_pda.to_account_info(),
                };
                let cpi_program = self.token_program.to_account_info();
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
                mint_to(cpi_ctx, 1_000_000)?;

                msg!("Minted 1,000,000 tokens to user");
            }
            _ => (),
        }

        let x = self.user_ata.owner;
        msg!("User ATA owner: {:?}", x);
        msg!("User public key: {:?}", self.user.key());
        
        Ok(())
    }

    pub fn associate_metadata(&mut self) -> Result<()> {
        let seeds = &[
            &b"counter"[..],
            &self.user.key().to_bytes()[..], 
            &[self.counter_pda.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let creator = vec![
            Creator {
                address: self.counter_pda.key(),
                verified: true,
                share: 100,
            },
        ];

        let metadata = &self.metadata.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.counter_pda.to_account_info();
        let payer = &self.user.to_account_info();
        let system_program = &self.system_program.to_account_info();

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            &self.token_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (&authority, true),
                system_program,
                rent: None,
            }, 
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: "Superteam UAE".to_string(),
                    symbol: "STUAE".to_string(),
                    uri: "".to_string(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: None,
                    uses: None
                },
                is_mutable: true,
                collection_details: None,
            }
        );
        metadata_account.invoke_signed(signer_seeds)?;

        Ok(())
    }

    pub fn close_ata(&mut self) -> Result<()> {
        let seeds = &[
            &b"counter"[..],
            &self.user.key().to_bytes()[..], 
            &[self.counter_pda.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.user.to_account_info(),
        };
        let cpi_program = self.system_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, self.user_ata.get_lamports())?;

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