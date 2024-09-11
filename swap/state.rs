use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Map;

pub struct Pool {
    pub token_a: Uint128,
    pub token_b: Uint128,
}

pub static POOLS: Map<(&str, &str), Pool> = Map::new("pools");
