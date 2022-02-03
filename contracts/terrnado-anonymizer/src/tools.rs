use cosmwasm_std::{Addr, Deps, MessageInfo, Response, StdError, StdResult, Uint128};
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
    return Err(ContractError::Unauthorized {});
}

pub fn convert_human_to_addr(
    deps: Deps,
    contracts_addresses: &Vec<String>,
) -> StdResult<Vec<Addr>> {
    contracts_addresses.iter()
        .map(|contract| -> StdResult<Addr> {
            let addr_form = deps.api.addr_validate(&contract);
            if addr_form.is_err() {
                Err::<(), StdError>(StdError::generic_err("Cannot convert address to addr"))?;
            }
            addr_form
        })
        .collect::<StdResult<Vec<Addr>>>()
}

pub fn get_sent_native_token_amount(info: &MessageInfo) -> Uint128 {
    match info.funds.iter().find(|x| x.denom == "uusd".to_string()) {
        Some(coin) => {
            coin.amount
        }
        None => {
            Uint128::zero()
        }
    }
}
