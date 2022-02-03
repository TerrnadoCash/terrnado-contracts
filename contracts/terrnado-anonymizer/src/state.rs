use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub deposits_number: u64,
    pub withdrawals_number: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    pub confirmed: bool,
    pub amount: Uint128,
    pub to: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const PROPOSAL_STATE: Item<u64> = Item::new("proposal_state");
pub const DEPOSITORS: Map<&Addr, bool> = Map::new("depositors");
pub const RELAYERS: Map<&Addr, bool> = Map::new("relayers");
pub const PROPOSALS: Map<String, Proposal> = Map::new("proposals");
