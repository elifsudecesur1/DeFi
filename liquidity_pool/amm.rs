use cosmwasm_std::Uint128;

pub fn calculate_k(token_a: Uint128, token_b: Uint128) -> Uint128 {
    token_a * token_b  
}
