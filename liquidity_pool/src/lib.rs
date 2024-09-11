use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::error::ContractError;
use crate::state::{LIQUIDITY_PROVIDERS, POOLS};

pub fn provide_liquidity(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_a_amount: Uint128,
    token_b_amount: Uint128,
) -> Result<Response, ContractError> {
    let provider = info.sender.clone();

    LIQUIDITY_PROVIDERS.update(deps.storage, &provider, |pool| -> Result<_, ContractError> {
        let mut pool = pool.unwrap_or_default();
        pool.token_a += token_a_amount;
        pool.token_b += token_b_amount;
        Ok(pool)
    })?;

    let k = calculate_k(token_a_amount, token_b_amount);

    Ok(Response::new()
        .add_attribute("action", "provide_liquidity")
        .add_attribute("provider", provider)
        .add_attribute("k", k))
}
