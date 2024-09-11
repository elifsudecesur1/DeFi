use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::error::ContractError;
use crate::state::{POOLS};
use crate::amm::calculate_swap;

pub fn swap_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    input_token: String,
    output_token: String,
) -> Result<Response, ContractError> {
    let pool = POOLS.load(deps.storage, (&input_token, &output_token))?;

    let swap_amount = calculate_swap(pool.token_a, pool.token_b, amount);

    POOLS.update(deps.storage, (&input_token, &output_token), |pool| -> Result<_, ContractError> {
        let mut pool = pool.unwrap();
        pool.token_a += amount;
        pool.token_b -= swap_amount;
        Ok(pool)
    })?;

    Ok(Response::new()
        .add_attribute("action", "swap_tokens")
        .add_attribute("input_token", input_token)
        .add_attribute("output_token", output_token)
        .add_attribute("amount_swapped", swap_amount))
}
