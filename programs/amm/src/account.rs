use anchor_lang::{prelude::*};

#[account]
pub struct Pool{
    pub lp_mint : Pubkey,
    pub total_liquidty : u64,
    pub token_a_mint : Pubkey , 
    pub token_b_mint : Pubkey , 
    pub token_a_vault_ata : Pubkey , 
    pub token_b_vault_ata : Pubkey , 
    // PDA Authority for lp mint, both vault ata 
    pub authority  : Pubkey,  
    pub bump  : u8,
    pub fee_rate : u8, 
}