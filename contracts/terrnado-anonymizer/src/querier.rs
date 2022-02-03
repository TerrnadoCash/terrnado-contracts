use cosmwasm_std::{QueryRequest, StdResult, BankQuery, QuerierWrapper, BalanceResponse};

pub fn load_ust_balance(
    querier: &QuerierWrapper,
    contract_addr: &String,
) -> StdResult<BalanceResponse> {
    querier
        .query(&QueryRequest::Bank(BankQuery::Balance {
            address: contract_addr.clone(),
            denom: String::from("uusd"),
        }))
}
