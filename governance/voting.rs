use cosmwasm_std::{Uint128, Decimal};

pub fn calculate_vote_weight(
    stake_amount: Uint128,
    total_stake: Uint128
) -> Decimal {
    Decimal::from_ratio(stake_amount, total_stake)
}
