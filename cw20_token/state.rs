use cosmwasm_std::{Addr, Uint128};
use std::sync::Mutex;

pub struct ReentrancyGuard {
    pub locked: bool,
}

impl ReentrancyGuard {
    pub fn new() -> Self {
        ReentrancyGuard { locked: false }
    }

    pub fn enter(&mut self) -> Result<(), String> {
        if self.locked {
            return Err("Reentrancy Error!".to_string());
        }
        self.locked = true;
        Ok(())
    }

    pub fn exit(&mut self) {
        self.locked = false;
    }
}

pub static REENTRANCY_GUARD: Mutex<ReentrancyGuard> = Mutex::new(ReentrancyGuard::new());

pub struct Balances {
    pub address: Addr,
    pub balance: Uint128,
}

pub static BALANCES: Map<Addr, Uint128> = Map::new("balances");
