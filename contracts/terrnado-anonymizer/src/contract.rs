use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use terrnado::terrnado_anonymizer::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};


use crate::errors::ContractError;
use crate::execute::{confirm_proposal, deposit, propose_withdraw, update_relayers};
use crate::queries::{query_config, query_relayer, query_state};
use crate::state::{Config, CONFIG, PROPOSAL_STATE, State, STATE};
use crate::tools::{assert_admin_privilege};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PROPOSAL_STATE.save(deps.storage, &0)?;
    CONFIG.save(deps.storage,
                &Config {
                    owner: info.sender,
                })?;

    STATE.save(deps.storage,
               &State {
                   deposits_number: 0,
                   withdrawals_number: 0,
               })?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg.clone() {
        ExecuteMsg::Deposit {} => deposit(deps, info),
        _ => {
            assert_admin_privilege(deps.as_ref(), info.clone())?;
            match msg {
                ExecuteMsg::UpdateRelayers { relayers } => update_relayers(deps, env, info, relayers),
                ExecuteMsg::ProposeWithdraw { amount, to } => propose_withdraw(deps, env, info, amount, to),
                ExecuteMsg::ConfirmWithdraw { proposal_id } => confirm_proposal(deps, env, info, proposal_id),
                _ => panic!("DO NOT ENTER HERE"),
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Relayer { address } => to_binary(&query_relayer(deps, address)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

