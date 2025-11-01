use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{spl_token::instruction::TokenInstruction, Mint, Token, TokenAccount }};


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


#[derive(Accounts)]
pub struct  Swap <'info>{
    pub user  :Signer<'info>,

    #[account(mut , associated_token::mint = token_a_mint , associated_token::authority = user)]
    pub user_token_a_ata : Account<'info , TokenAccount>,
    #[account(mut , associated_token::mint = token_b_mint , associated_token::authority = user)]
    pub user_token_b_ata : Account<'info , TokenAccount>,


    #[account(seeds=[b"pool",
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

    pub token_a_mint : Account<'info , Mint>,
    pub token_b_mint : Account<'info , Mint>,
    
    pub token_program : Program<'info , Token>
}



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