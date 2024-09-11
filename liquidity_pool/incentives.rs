use cosmwasm_std::{DepsMut, Addr, Uint128, Decimal};
use crate::state::{LIQUIDITY_PROVIDERS, REWARDS};

pub fn calculate_incentives(
    deps: DepsMut,
    provider: Addr,
    total_liquidity: Uint128,
    reward_rate: Decimal,
) -> Result<Response, ContractError> {
    let pool = LIQUIDITY_PROVIDERS.load(deps.storage, &provider)?;
    let provider_liquidity = pool.token_a + pool.token_b;
    
    let reward = provider_liquidity * reward_rate / total_liquidity;

    REWARDS.update(deps.storage, &provider, |balance| -> Result<_, ContractError> {
        let mut balance = balance.unwrap_or_default();
        balance += reward;
        Ok(balance)
    })?;

    Ok(Response::new()
        .add_attribute("action", "calculate_incentives")
        .add_attribute("provider", provider)
        .add_attribute("reward", reward))
}
