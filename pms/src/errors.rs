use anchor_lang::prelude::*;

#[error_code]
pub enum PerpError {
    #[msg("Math overflow")] Overflow,
    #[msg("Math underflow")] Underflow,
    #[msg("Division by zero")] DivisionByZero,
    #[msg("Invalid leverage")] InvalidLeverage,
    #[msg("Leverage exceeded for tier")] LeverageExceeded,
    #[msg("Invalid position size")] InvalidSize,
    #[msg("Invalid amount")] InvalidAmount,
    #[msg("Symbol too long")] SymbolTooLong,
    #[msg("Insufficient margin for increase")] InsufficientMarginForIncrease,
    #[msg("Post-removal margin would breach maintenance")] MaintenanceBreach,
    #[msg("Invalid state")] InvalidState,
}