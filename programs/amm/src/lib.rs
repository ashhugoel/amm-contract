use anchor_lang::prelude::*;
mod account;
use account::*;

pub mod instructions;      
use instructions::*;        

declare_id!("HrBrzXye9KcPPdpcWHwAsKF4dHZyNTtMWTD2F6wvSxod");

#[program]
pub mod amm {

    use anchor_spl::token::{self, transfer, Burn, MintTo, Transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle(ctx)?;
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount_a: u64, amount_b: u64) -> Result<()> {
        addliquidity(ctx , amount_a,  amount_b)?;
        Ok(())
    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>, lp_amount: u64) -> Result<()> {
        removeliquidity(ctx ,  lp_amount)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_a_to_b: bool, amount: u64) -> Result<()> {
        swapamm(ctx, is_a_to_b, amount)?;
        Ok(())
    }
}
