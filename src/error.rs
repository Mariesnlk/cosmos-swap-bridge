use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient Funds")]
    InsufficientFunds {},

    #[error("Failed Swap: {reason:?}")]
    FailedSwap { reason: String },

}