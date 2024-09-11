use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Insufficient Liquidity")]
    InsufficientLiquidity {},

    #[error("Pool Not Found")]
    PoolNotFound {},

    #[error("Unauthorized")]
    Unauthorized {},
}
