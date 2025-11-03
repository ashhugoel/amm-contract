use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("Low balance in user token A ata")]
    LowBalanceInUserTokenAATA,

    #[msg("Low balance in user token B ata")]
    LowBalanceInUserTokenBATA,
    
}