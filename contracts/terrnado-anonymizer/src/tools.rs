use cosmwasm_std::{Deps, MessageInfo, Response, Uint128};
use crate::errors::ContractError;
use crate::state::{CONFIG, RELAYERS};

pub fn assert_admin_privilege(
    deps: Deps,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    if RELAYERS.load(deps.storage, &info.sender).unwrap_or(false) {
        return Ok(Response::default());
    }
    if CONFIG.load(deps.storage)?.owner == info.sender {
        return Ok(Response::default());
    }
    Err(ContractError::Unauthorized {})
}

pub fn get_sent_native_token_amount(info: &MessageInfo) -> Uint128 {
    match info.funds.iter().find(|x| x.denom == *"uusd") {
        Some(coin) => {
            coin.amount
        }
        None => {
            Uint128::zero()
        }
    }
}
