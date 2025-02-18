pub mod states;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

// declare_id!("3x7vyDWQLvY2nBuxCWKRDd7tewBrihiwBRSxZJUumdTX");
declare_id!("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c");

#[program]
pub mod lifinity_cpi_swap {
    use super::*;

    pub fn swap(ctx: Context<Swap>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        let swap_ix = solana_program::instruction::Instruction {
            program_id: ctx.accounts.lifinity_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.pool_authority.key(), false),
                AccountMeta::new(ctx.accounts.amm.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user_transfer_authority.key(), true),
                AccountMeta::new(ctx.accounts.source_info.key(), false),
                AccountMeta::new(ctx.accounts.destination_info.key(), false),
                AccountMeta::new(ctx.accounts.swap_source.key(), false),
                AccountMeta::new(ctx.accounts.swap_destination.key(), false),
                AccountMeta::new(ctx.accounts.pool_mint.key(), false),
                AccountMeta::new(ctx.accounts.fee_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.oracle_main.key(), false),
                AccountMeta::new_readonly(ctx.accounts.oracle_sub.key(), false),
                AccountMeta::new_readonly(ctx.accounts.oracle_pc.key(), false),
            ],
            data: lifinity_cpi_swap::instruction::Swap {
                amount_in,
                minimum_amount_out: min_amount_out,
            }
              .try_to_vec()?,
        };

        invoke_signed(
            &swap_ix,
            &[
                ctx.accounts.pool_authority.to_account_info(),
                ctx.accounts.amm.to_account_info(),
                ctx.accounts.user_transfer_authority.to_account_info(),
                ctx.accounts.source_info.to_account_info(),
                ctx.accounts.destination_info.to_account_info(),
                ctx.accounts.swap_source.to_account_info(),
                ctx.accounts.swap_destination.to_account_info(),
                ctx.accounts.pool_mint.to_account_info(),
                ctx.accounts.fee_account.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.oracle_main.to_account_info(),
                ctx.accounts.oracle_sub.to_account_info(),
                ctx.accounts.oracle_pc.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub lifinity_program: AccountInfo<'info>,
    pub pool_authority: AccountInfo<'info>,
    pub amm: AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub source_info: Account<'info, TokenAccount>,
    pub destination_info: Account<'info, TokenAccount>,
    pub swap_source: Account<'info, TokenAccount>,
    pub swap_destination: Account<'info, TokenAccount>,
    pub pool_mint: Account<'info, TokenAccount>,
    pub fee_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub oracle_main: AccountInfo<'info>,
    pub oracle_sub: AccountInfo<'info>,
    pub oracle_pc: AccountInfo<'info>,
}
