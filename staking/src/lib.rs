use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, Decimal};
use crate::state::{STAKED_TOKENS, REWARDS};

pub fn stake_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    lockup_period: u64,
) -> Result<Response, ContractError> {
    let staker = info.sender.clone();

    STAKED_TOKENS.update(deps.storage, &staker, |stake| -> Result<_, ContractError> {
        let mut stake = stake.unwrap_or_default();
        stake.amount += amount;
        stake.lockup_end = env.block.time.plus_seconds(lockup_period);
        Ok(stake)
    })?;

    Ok(Response::new()
        .add_attribute("action", "stake_tokens")
        .add_attribute("staker", staker)
        .add_attribute("amount", amount))
}
