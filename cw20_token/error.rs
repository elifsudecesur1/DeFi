use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Insufficient Funds")]
    InsufficientFunds {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Reentrancy Guard Error")]
    ReentrancyError {},
}
