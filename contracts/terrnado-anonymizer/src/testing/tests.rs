use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, BankMsg, coin, Coin, CosmosMsg, from_binary, SubMsg, Uint128};
use terrnado::terrnado_anonymizer::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, RelayerResponse, StateResponse};
use crate::contract::{execute, instantiate, query};

use crate::errors::ContractError;

#[test]
fn initialization() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("sender", &vec![]);
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    assert_eq!(
        from_binary::<ConfigResponse>(&query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()).unwrap(),
        ConfigResponse {
            owner: String::from("sender"),
        }
    );
}

#[test]
fn add_relayers1() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("sender", &vec![]);
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let msg = ExecuteMsg::UpdateRelayers {
        relayers: vec![String::from("relayer1"), String::from("relayer2")]
    };
    let info = mock_info("sender2", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("sender", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "update_relayers")]);

    assert_eq!(
        from_binary::<RelayerResponse>(&query(deps.as_ref(), mock_env(), QueryMsg::Relayer { address: String::from("relayer1") }).unwrap()).unwrap(),
        RelayerResponse {
            relayer: String::from("relayer1"),
            is_relayer: true,
        }
    );
    assert_eq!(
        from_binary::<RelayerResponse>(&query(deps.as_ref(), mock_env(), QueryMsg::Relayer { address: String::from("relayer2") }).unwrap()).unwrap(),
        RelayerResponse {
            relayer: String::from("relayer2"),
            is_relayer: true,
        }
    );
    assert_eq!(
        from_binary::<RelayerResponse>(&query(deps.as_ref(), mock_env(), QueryMsg::Relayer { address: String::from("relayer3") }).unwrap()).unwrap(),
        RelayerResponse {
            relayer: String::from("relayer3"),
            is_relayer: false,
        }
    );

    let msg = ExecuteMsg::UpdateRelayers {
        relayers: vec![String::from("relayer3"), String::from("relayer4")]
    };
    let info = mock_info("relayer1", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}

#[test]
fn deposit() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("sender", &vec![]);
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let info = mock_info("otherAddr", &vec![coin(10000000, "uusd")]);
    let msg = ExecuteMsg::Deposit {};
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::WrongDeposit {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    // Success
    let info = mock_info("otherAddr", &vec![coin(100000000, "uusd")]);
    let msg = ExecuteMsg::Deposit {};
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "deposit"),
                                    attr("user", "otherAddr"),
                                    attr("amount", "100000000"),
                                    attr("index", "1"),
    ]);
    assert_eq!(res.messages.len(), 0);


    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.messages, vec![]);
    assert_eq!(res.attributes, vec![attr("action", "deposit"),
                                    attr("user", "otherAddr"),
                                    attr("amount", "100000000"),
                                    attr("index", "2"),
    ]);

    assert_eq!(
        from_binary::<StateResponse>(&query(deps.as_ref(), mock_env(), QueryMsg::State {}).unwrap()).unwrap(),
        StateResponse {
            deposits_number: 2,
            unique_users: 1,
            withdrawals_number: 0,
        }
    );
}

#[test]
fn propose_withdraw() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("sender", &vec![]);
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let info = mock_info("otherAddr", &vec![coin(100000000, "uusd")]);
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

    let msg = ExecuteMsg::UpdateRelayers {
        relayers: vec![String::from("relayer1"), String::from("relayer2")]
    };
    let info = mock_info("sender", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "update_relayers")]);

    //propose
    let info = mock_info("otherAddr", &vec![]);
    let msg = ExecuteMsg::ProposeWithdraw {
        amount: Uint128::from(100000000u128),
        to: String::from("destination"),
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    let info = mock_info("relayer1", &vec![]);
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "propose_withdraw"),
                                    attr("amount", "100000000"),
                                    attr("to", "destination"),
                                    attr("proposal_id", "1"),
    ]);
    assert_eq!(res.messages.len(), 0);

    // assert_eq!(
    //     res.messages,
    //     vec![
    //         SubMsg::new(
    //             CosmosMsg::Bank(BankMsg::Send {
    //                 to_address: info.sender.clone().into_string(),
    //                 amount: vec![Coin { denom: String::from("uusd"), amount: Uint128::from(9900990u128) }],
    //             })
    //         )
    //     ],
    // );
}

#[test]
fn confirm_withdraw() {
    let mut deps = mock_dependencies(&[]);
    let info = mock_info("sender", &vec![]);
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let info = mock_info("otherAddr", &vec![coin(100000000, "uusd")]);
    let msg = ExecuteMsg::Deposit {};
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

    let msg = ExecuteMsg::UpdateRelayers {
        relayers: vec![String::from("relayer1"), String::from("relayer2")]
    };
    let info = mock_info("sender", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

    //propose
    let msg = ExecuteMsg::ProposeWithdraw {
        amount: Uint128::from(100000000u128),
        to: String::from("destination"),
    };

    let info = mock_info("relayer1", &vec![]);
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();


    //confirm
    let info = mock_info("otherAddr", &vec![]);
    let msg = ExecuteMsg::ConfirmWithdraw {
        proposal_id: 1,
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    //confirm
    let info = mock_info("sender", &vec![]);
    let msg = ExecuteMsg::ConfirmWithdraw {
        proposal_id: 2,
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    match res {
        ContractError::NotFound {} => {}
        _ => panic!("DO NOT ENTER HERE"),
    }

    //confirm
    let info = mock_info("relayer1", &vec![]);
    let msg = ExecuteMsg::ConfirmWithdraw {
        proposal_id: 1,
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
    assert_eq!(res.attributes, vec![attr("action", "confirm_withdraw"),
                                    attr("proposal_id", "1"),
                                    attr("amount", "100000000"),
                                    attr("to", "destination"),
    ]);
    assert_eq!(
        res.messages,
        vec![
            SubMsg::new(
                CosmosMsg::Bank(BankMsg::Send {
                    to_address: String::from("destination"),
                    amount: vec![Coin { denom: String::from("uusd"), amount: Uint128::from(100000000u128) }],
                })
            )
        ],
    );

    let info = mock_info("relayer1", &vec![]);
    let msg = ExecuteMsg::ConfirmWithdraw {
        proposal_id: 1,
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
    match res {
        ContractError::AlreadyWithdrawn {} => {}
        _ => panic!("DO NOT ENTER HERE"),
    }
}
