use anchor_lang::prelude::*;
use anchor_spl::{ token::{self, transfer,  Mint, Token, TokenAccount ,Transfer ,MintTo }};
use crate::account::Pool;
use crate::errors::MyError;

#[derive(Accounts)]
pub struct AddLiquidity<'info>{
    pub user : Signer<'info> , 

    #[account(mut , associated_token::mint = token_a_mint, associated_token::authority = user)]
    pub user_token_a_ata : Account<'info,  TokenAccount>,
    #[account(mut , associated_token::mint = token_b_mint, associated_token::authority = user)]
    pub user_token_b_ata : Account<'info,  TokenAccount>,

    #[account(mut , associated_token::mint = lp_mint, associated_token::authority = user)]
    pub user_lp_ata : Account<'info,  TokenAccount>,

    #[account(mut ,seeds=[b"pool" , token_a_mint.key().as_ref(),  token_b_mint.key().as_ref() ] ,bump ,
    has_one = token_a_mint, 
    has_one = token_b_mint,
    has_one = lp_mint,
    has_one = authority)]
    pub pool_account  : Account<'info , Pool>,

    #[account(seeds=[b"authority" , pool_account.key().as_ref()], bump)]
    /// CHECK: pda  
    pub authority : UncheckedAccount<'info>,

    #[account(mut , associated_token::mint= token_a_mint, associated_token::authority = authority)]
    pub token_a_vault_ata : Account<'info , TokenAccount>,
    #[account(mut ,associated_token::mint= token_b_mint, associated_token::authority = authority)]
    pub token_b_vault_ata : Account<'info , TokenAccount>,

    #[account(mut)]
    pub lp_mint  : Account<'info , Mint>,
    pub token_a_mint  : Account<'info , Mint>,
    pub token_b_mint  : Account<'info , Mint>,
    pub token_program  : Program<'info , Token>
}


pub fn addliquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
   
    require!(ctx.accounts.user_token_a_ata.amount>= amount_a , MyError::LowBalanceInUserTokenAATA);
    require!(ctx.accounts.user_token_b_ata.amount>= amount_b , MyError::LowBalanceInUserTokenBATA);
    
    let pool = &mut ctx.accounts.pool_account;

    let decimal_mint_a = ctx.accounts.token_a_mint.decimals;
    let decimal_mint_b = ctx.accounts.token_b_mint.decimals;
    let lp_mint = ctx.accounts.lp_mint.decimals;

    let mut amount_a = (amount_a as f64) / 10f64.powi(decimal_mint_a as i32);
    let mut amount_b = (amount_b as f64) / 10f64.powi(decimal_mint_b as i32);

    //will be used outside if too so
    let pool_key = pool.key();
    let seed: &[&[u8]] = &[b"authority", pool_key.as_ref(), &[pool.bump]];
    let signer = &[&seed[..]];

    if pool.total_liquidty == 0 {
        let lp_token = (amount_a * amount_b).sqrt();

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer,
        );

        token::mint_to(cpi_ctx, (lp_token * 10f64.powf(lp_mint as f64)) as u64)?;

        let cpi_ctx_1 = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_a_ata.to_account_info(),
                to: ctx.accounts.token_a_vault_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );


        transfer(
            cpi_ctx_1,
            (amount_a * 10f64.powi(decimal_mint_a as i32)) as u64,
        )?;
       

        let cpi_ctx_2 = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_b_ata.to_account_info(),
                to: ctx.accounts.token_b_vault_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        transfer(
            cpi_ctx_2,
            (amount_b * 10f64.powi(decimal_mint_b as i32)) as u64,
        )?;

        pool.total_liquidty = (lp_token * 10f64.powf(lp_mint as f64)) as u64;
        return Ok(());
    }

    let bal_vault_a =
        ctx.accounts.token_a_vault_ata.amount as f64 / 10f64.powi(decimal_mint_a as i32);
    let bal_vault_b =
        ctx.accounts.token_b_vault_ata.amount as f64 / 10f64.powi(decimal_mint_b as i32);

    let ratio_pool: f64 = bal_vault_a as f64 / bal_vault_b as f64;

    let possible_a = amount_b * ratio_pool;
    let possible_b = amount_a / ratio_pool;

    if possible_a <= amount_a {
        amount_a = possible_a;
        amount_b = amount_b;
    } else {
        amount_a = amount_a;
        amount_b = possible_b;
    }

    let total_liquidity_human = pool.total_liquidty as f64 / 10f64.powi(lp_mint as i32);
    let lp_token_mint = total_liquidity_human * (amount_a / bal_vault_a as f64);

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.lp_mint.to_account_info(),
            to: ctx.accounts.user_lp_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
        signer,
    );
    token::mint_to(cpi_ctx, (lp_token_mint * 10f64.powf(lp_mint as f64)) as u64)?;

    let cpi_ctx_1 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_a_ata.to_account_info(),
            to: ctx.accounts.token_a_vault_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    transfer(
        cpi_ctx_1,
        (amount_a * 10f64.powi(decimal_mint_a as i32)) as u64,
    )?;

    let cpi_ctx_2 = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_token_b_ata.to_account_info(),
            to: ctx.accounts.token_b_vault_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    transfer(
        cpi_ctx_2,
        (amount_b * 10f64.powi(decimal_mint_b as i32)) as u64,
    )?;
    pool.total_liquidty += (lp_token_mint * 10f64.powf(lp_mint as f64)) as u64;

    Ok(())
}