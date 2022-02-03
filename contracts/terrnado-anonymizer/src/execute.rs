use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use crate::errors::ContractError;
use crate::state::{CONFIG, DEPOSITORS, Proposal, PROPOSAL_STATE, PROPOSALS, RELAYERS, STATE};
use crate::tools::get_sent_native_token_amount;

pub fn deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let sent_ust_amount = get_sent_native_token_amount(&info);

    if sent_ust_amount != Uint128::from(100000000u128)
        && sent_ust_amount != Uint128::from(1000000000u128)
        && sent_ust_amount != Uint128::from(10000000000u128) {
        return Err(ContractError::WrongDeposit {});
    }

    let registered = DEPOSITORS.load(deps.storage, &info.sender).unwrap_or(false);
    if !registered {
        DEPOSITORS.save(deps.storage, &info.sender, &true)?;
    }

    let mut deposit_number = 0;
    STATE.update(deps.storage, |a| -> StdResult<_> {
        let mut new_state = a;
        new_state.deposits_number += 1;
        deposit_number = new_state.deposits_number;
        Ok(new_state)
    })?;

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("user", info.sender)
        .add_attribute("amount", sent_ust_amount)
        .add_attribute("index", deposit_number.to_string())
    )
}

pub fn confirm_proposal(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    proposal_id: u64,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |a| -> StdResult<_> {
        let mut new_state = a;
        new_state.withdrawals_number += 1;
        Ok(new_state)
    })?;

    let proposal = PROPOSALS.load(deps.storage, proposal_id.to_string()).unwrap_or(Proposal {
        confirmed: false,
        to: Addr::unchecked(""),
        amount: Uint128::zero(),
    });
    if proposal.amount == Uint128::zero() {
        return Err(ContractError::NotFound {});
    }
    if proposal.confirmed {
        return Err(ContractError::AlreadyWithdrawn {});
    }
    PROPOSALS.update(deps.storage, proposal_id.to_string(), |a| -> StdResult<_> {
        let mut proposal = a.unwrap();
        proposal.confirmed = true;
        Ok(proposal)
    })?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(
            BankMsg::Send {
                to_address: proposal.to.clone().to_string(),
                amount: vec![Coin { amount: proposal.amount.clone(), denom: String::from("uusd") }],
            },
        ))
        .add_attribute("action", "confirm_withdraw")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("amount", proposal.amount.to_string())
        .add_attribute("to", proposal.to.to_string())
    )
}

pub fn propose_withdraw(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
    to: String,
) -> Result<Response, ContractError> {
    let proposal_id = PROPOSAL_STATE.load(deps.storage)? + 1;
    PROPOSALS.save(deps.storage, proposal_id.clone().to_string(), &Proposal {
        amount,
        to: deps.api.addr_validate(&to)?,
        confirmed: false,
    })?;
    PROPOSAL_STATE.save(deps.storage, &proposal_id)?;

    Ok(Response::new()
        .add_attribute("action", "propose_withdraw")
        .add_attribute("amount", amount)
        .add_attribute("to", to)
        .add_attribute("proposal_id", proposal_id.to_string())
    )
}

pub fn update_relayers(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    relayers: Vec<String>,
) -> Result<Response, ContractError> {
    if CONFIG.load(deps.storage)?.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    for relay in relayers {
        RELAYERS.save(deps.storage, &deps.api.addr_validate(&relay)?, &true)?;
    }

    Ok(Response::new()
        .add_attribute("action", "update_relayers")
    )
}
