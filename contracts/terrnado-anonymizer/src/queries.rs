use cosmwasm_std::{StdResult, Deps, Order};
use terrnado::terrnado_anonymizer::{ConfigResponse, RelayerResponse, StateResponse};
use crate::state::{CONFIG, DEPOSITORS, RELAYERS, STATE};

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: state.owner.into_string(),
    })
}

pub fn query_state(
    deps: Deps,
) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;

    let users_number = DEPOSITORS.keys(deps.storage, None, None, Order::Ascending).count();

    Ok(StateResponse {
        deposits_number: state.deposits_number,
        withdrawals_number: state.withdrawals_number,
        unique_users: users_number,
    })
}

pub fn query_relayer(
    deps: Deps,
    relayer: String,
) -> StdResult<RelayerResponse> {
    let is_relayer = RELAYERS.load(deps.storage, &deps.api.addr_validate(&relayer)?).unwrap_or(false);

    Ok(RelayerResponse {
        relayer,
        is_relayer,
    })
}
