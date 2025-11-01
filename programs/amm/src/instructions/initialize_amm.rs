use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{ Mint, Token, TokenAccount }};

use crate::account::Pool;

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub payer : Signer<'info>,

    #[account(init, 
        seeds=[b"pool", token_a_mint.key().as_ref(),token_b_mint.key().as_ref()],
        payer=payer,    
        bump,
        space = 210
        )]
    pub pool_account : Account<'info , Pool>,

    #[account(seeds=[b"authority" , pool_account.key().as_ref()], bump)]
    /// CHECK: pda auth for all three thingy
    pub authority  : UncheckedAccount<'info>,


    #[account(init , payer= payer , associated_token::mint= token_a_mint, associated_token::authority = authority)]
    pub token_a_vault_ata : Account<'info , TokenAccount>,
    #[account(init , payer= payer , associated_token::mint= token_b_mint, associated_token::authority = authority)]
    pub token_b_vault_ata : Account<'info , TokenAccount>,

    
    #[account(init , payer = payer ,
    seeds=[b"mint", pool_account.key().as_ref()],
    bump ,
    mint::decimals = 6 ,
    mint::authority = authority, 
    )]
    pub lp_mint : Account<'info , Mint>,


    pub token_a_mint : Account<'info , Mint>,
    pub token_b_mint : Account<'info , Mint>,

    pub associated_token_program : Program<'info , AssociatedToken>,
    pub token_program : Program<'info, Token>,
    pub system_program  : Program<'info , System>
}


pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let (pool_authority, bump) = Pubkey::find_program_address(
        &[b"authority", ctx.accounts.pool_account.key().as_ref()],
        ctx.program_id,
    );

    let pool = &mut ctx.accounts.pool_account;
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.total_liquidty = 0;
    pool.token_a_mint = ctx.accounts.token_a_mint.key();
    pool.token_b_mint = ctx.accounts.token_b_mint.key();
    pool.token_a_vault_ata = ctx.accounts.token_a_vault_ata.key();
    pool.token_b_vault_ata = ctx.accounts.token_b_vault_ata.key();

    pool.authority = ctx.accounts.authority.key();
    pool.bump = bump;
    pool.fee_rate = 30;

    Ok(())
}
