use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("the given name is too long")]
    NameTooLong,
}