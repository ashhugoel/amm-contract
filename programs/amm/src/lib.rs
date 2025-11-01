use anchor_lang::prelude::*;
mod account;
use account::*;

declare_id!("HrBrzXye9KcPPdpcWHwAsKF4dHZyNTtMWTD2F6wvSxod");

#[program]
pub mod amm {

    use anchor_spl::token::{self, transfer, Burn, MintTo, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
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

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
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

            msg!("in if block 3 {}");

            transfer(
                cpi_ctx_1,
                (amount_a * 10f64.powi(decimal_mint_a as i32)) as u64,
            )?;
            msg!(
                "in if block 4 {}",
                amount_a * 10f64.powi(decimal_mint_a as i32)
            );

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

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
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

    pub fn swap(ctx: Context<Swap>, is_a_to_b: bool, amount: u64) -> Result<()> {
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
                Transfer{
                    from:ctx.accounts.token_b_vault_ata.to_account_info(),
                    to: ctx.accounts.user_token_b_ata.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }, signer);

            msg!("Amount a:{}, supply b:{}, supply a:{},  k:{}",amount_a, supply_b , supply_a , k);
            msg!("Amount b to be added in {:?}",(swap_amount_b * 10f64.powi(decimal_mint_b as i32)) as u64);
            transfer(cpi_ctx_outvault, (swap_amount_b * 10f64.powi(decimal_mint_b as i32)) as u64)?;
        }else{
            let amount_b = amount as f64 / 10f64.powi(decimal_mint_b as i32);
            
            let swap_amount_a = supply_a - k / (supply_b + amount_b);

            let cpi_invault = CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
            Transfer{
                from: ctx.accounts.user_token_b_ata.to_account_info(),
                to: ctx.accounts.token_b_vault_ata.to_account_info(),
                authority:ctx.accounts.user.to_account_info(),
            });

            transfer(cpi_invault, amount)?;

            let cpi_outvault = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                Transfer{
                    from:ctx.accounts.token_a_vault_ata.to_account_info(),
                    to:ctx.accounts.user_token_a_ata.to_account_info(),
                    authority:ctx.accounts.authority.to_account_info()
                }, signer);

            transfer(cpi_outvault, (swap_amount_a * 10f64.powi(decimal_mint_a as i32)) as u64)?;
        }

        Ok(())
    }
}
