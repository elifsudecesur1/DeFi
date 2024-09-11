use cosmwasm_std::Uint128;
use crate::state::{STAKED_TOKENS, REWARDS};

pub fn calculate_rewards(
    stake_amount: Uint128,
    stake_duration: u64,
    reward_rate: Decimal,
) -> Uint128 {
    stake_amount * reward_rate * stake_duration
}
