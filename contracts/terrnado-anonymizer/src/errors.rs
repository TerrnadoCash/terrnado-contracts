use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Deposit 100, 1k or 10k ust")]
    WrongDeposit {},

    #[error("Already withdrawn")]
    AlreadyWithdrawn {},

    #[error("Not Found")]
    NotFound {},
}
