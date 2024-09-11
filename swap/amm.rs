use cosmwasm_std::Uint128;

pub fn calculate_swap(
    token_a_amount: Uint128,
    token_b_amount: Uint128,
    swap_amount: Uint128,
) -> Uint128 {
    let new_token_a_amount = token_a_amount + swap_amount;
    let new_token_b_amount = token_b_amount * token_a_amount / new_token_a_amount;
    token_b_amount - new_token_b_amount
}
