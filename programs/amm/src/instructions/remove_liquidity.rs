use anchor_lang::{prelude::*};
use anchor_spl::{ token::{self, transfer,  Mint, Token, TokenAccount ,Transfer , Burn }};
use crate::account::Pool;



#[derive(Accounts)]
pub struct  RemoveLiquidity <'info>{
    pub user  :Signer<'info>,

    #[account(mut , associated_token::mint = token_a_mint , associated_token::authority = user)]
    pub user_token_a_ata : Account<'info , TokenAccount>,
    #[account(mut , associated_token::mint = token_b_mint , associated_token::authority = user)]
    pub user_token_b_ata : Account<'info , TokenAccount>,
    #[account(mut , associated_token::mint = lp_mint , associated_token::authority = user)]
    pub user_lp_ata : Account<'info , TokenAccount>,

    #[account(mut , seeds=[b"pool",
        token_a_mint.key().as_ref(),
        token_b_mint.key().as_ref()],
        bump, has_one = authority)]
    pub pool_account: Account<'info, Pool>,

    #[account(seeds = [b"authority", pool_account.key().as_ref()], bump )]
    ///CHECK : pda for signing
    pub authority : UncheckedAccount<'info>,

    #[account(mut ,associated_token::mint = token_a_mint , associated_token::authority = authority)]
    pub token_a_vault_ata : Account<'info , TokenAccount>,

    #[account(mut ,associated_token::mint = token_b_mint , associated_token::authority = authority)]
    pub token_b_vault_ata : Account<'info , TokenAccount>,


    #[account(mut , seeds=[b"mint", pool_account.key().as_ref()],bump)]
    pub lp_mint: Account<'info, Mint>,
    pub token_a_mint : Account<'info , Mint>,
    pub token_b_mint : Account<'info , Mint>,
    
    pub token_program : Program<'info , Token>
}



pub fn removeliquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool_account;
    let pool_key = pool.key();
    let total_liquidty = pool.total_liquidty;

    let proportional_share = lp_amount as f64 / total_liquidty as f64;

    let decimal_mint_a = ctx.accounts.token_a_mint.decimals;
    let decimal_mint_b = ctx.accounts.token_b_mint.decimals;
    // let decimal_mint_lp = ctx.accounts.lp_mint.decimals;

    let total_amount_a =
        ctx.accounts.token_a_vault_ata.amount as f64 / 10f64.powi(decimal_mint_a as i32);
    let total_amount_b =
        ctx.accounts.token_b_vault_ata.amount as f64 / 10f64.powi(decimal_mint_b as i32);
    // let lp_amount_cal = lp_amount as f64 / 10f64.powi(decimal_mint_lp as i32);

    let amount_a = total_amount_a * proportional_share;
    let amount_b = total_amount_b * proportional_share;

    let seed: &[&[u8]] = &[b"authority", pool_key.as_ref(), &[pool.bump]];
    let signer = &[&seed[..]];

    let mint_burn_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.lp_mint.to_account_info(),
            from: ctx.accounts.user_lp_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    token::burn(mint_burn_ctx, lp_amount)?;

    let cpi_ctx_a = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.token_a_vault_ata.to_account_info(),
            to: ctx.accounts.user_token_a_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );

    transfer(
        cpi_ctx_a,
        (amount_a * 10f64.powi(decimal_mint_a as i32)) as u64,
    )?;

    let cpi_ctx_b = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.token_b_vault_ata.to_account_info(),
            to: ctx.accounts.user_token_b_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );
    transfer(
        cpi_ctx_b,
        (amount_b * 10f64.powi(decimal_mint_b as i32)) as u64,
    )?;

    pool.total_liquidty -= lp_amount;

    Ok(())
}

  