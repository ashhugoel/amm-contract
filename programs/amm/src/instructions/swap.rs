use crate::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::{ transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Swap<'info> {
    pub user: Signer<'info>,

    #[account(mut , associated_token::mint = token_a_mint , associated_token::authority = user)]
    pub user_token_a_ata: Account<'info, TokenAccount>,
    #[account(mut , associated_token::mint = token_b_mint , associated_token::authority = user)]
    pub user_token_b_ata: Account<'info, TokenAccount>,

    #[account(seeds=[b"pool",
        token_a_mint.key().as_ref(),
        token_b_mint.key().as_ref()],
        bump, has_one = authority)]
    pub pool_account: Account<'info, Pool>,

    #[account(seeds = [b"authority", pool_account.key().as_ref()], bump )]
    ///CHECK : pda for signing
    pub authority: UncheckedAccount<'info>,

    #[account(mut ,associated_token::mint = token_a_mint , associated_token::authority = authority)]
    pub token_a_vault_ata: Account<'info, TokenAccount>,

    #[account(mut ,associated_token::mint = token_b_mint , associated_token::authority = authority)]
    pub token_b_vault_ata: Account<'info, TokenAccount>,

    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn swapamm(ctx: Context<Swap>, is_a_to_b: bool, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool_account;

    let decimal_mint_a = ctx.accounts.token_a_mint.decimals;
    let decimal_mint_b = ctx.accounts.token_b_mint.decimals;

    let supply_a = ctx.accounts.token_a_vault_ata.amount as f64 / 10f64.powi(decimal_mint_a as i32);
    let supply_b = ctx.accounts.token_b_vault_ata.amount as f64 / 10f64.powi(decimal_mint_b as i32);
    let k = supply_a * supply_b;

    let pool_key = pool.key();
    let seed: &[&[u8]] = &[b"authority", pool_key.as_ref(), &[pool.bump]];
    let signer = &[&seed[..]];

    if is_a_to_b {
        let amount_a = amount as f64 / 10f64.powi(decimal_mint_a as i32);

        let swap_amount_b = supply_b - (k / (supply_a + amount_a));

        let cpi_ctx_invault = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_a_ata.to_account_info(),
                to: ctx.accounts.token_a_vault_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        transfer(cpi_ctx_invault, amount)?;

        let cpi_ctx_outvault = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_b_vault_ata.to_account_info(),
                to: ctx.accounts.user_token_b_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer,
        );

        msg!(
            "Amount a:{}, supply b:{}, supply a:{},  k:{}",
            amount_a,
            supply_b,
            supply_a,
            k
        );
        msg!(
            "Amount b to be added in {:?}",
            (swap_amount_b * 10f64.powi(decimal_mint_b as i32)) as u64
        );
        transfer(
            cpi_ctx_outvault,
            (swap_amount_b * 10f64.powi(decimal_mint_b as i32)) as u64,
        )?;
    } else {
        let amount_b = amount as f64 / 10f64.powi(decimal_mint_b as i32);

        let swap_amount_a = supply_a - k / (supply_b + amount_b);

        let cpi_invault = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_b_ata.to_account_info(),
                to: ctx.accounts.token_b_vault_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );

        transfer(cpi_invault, amount)?;

        let cpi_outvault = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_a_vault_ata.to_account_info(),
                to: ctx.accounts.user_token_a_ata.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer,
        );

        transfer(
            cpi_outvault,
            (swap_amount_a * 10f64.powi(decimal_mint_a as i32)) as u64,
        )?;
    }

    Ok(())
}
