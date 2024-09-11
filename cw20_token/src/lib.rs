use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, Decimal, Addr};
use crate::error::ContractError;
use crate::state::{BALANCES, REENTRANCY_GUARD};

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    burn_rate: Decimal,
    commission_rate: Decimal,
    commission_address: Addr,
) -> Result<Response, ContractError> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;
    let sender_addr = info.sender.clone();

    let burn_amount = amount * burn_rate;
    let commission_amount = amount * commission_rate;
    let transfer_amount = amount - burn_amount - commission_amount;

    let mut guard = REENTRANCY_GUARD.lock();
    guard.enter()?;

    BALANCES.update(deps.storage, &sender_addr, |balance| -> Result<_, ContractError> {
        let mut balance = balance.unwrap_or_default();
        if balance < amount {
            return Err(ContractError::InsufficientFunds {});
        }
        balance -= amount;
        Ok(balance)
    })?;

    BALANCES.update(deps.storage, &recipient_addr, |balance| -> Result<_, ContractError> {
        let mut balance = balance.unwrap_or_default();
        balance += transfer_amount;
        Ok(balance)
    })?;

    BALANCES.update(deps.storage, &commission_address, |balance| -> Result<_, ContractError> {
        let mut balance = balance.unwrap_or_default();
        balance += commission_amount;
        Ok(balance)
    })?;

    guard.exit();

    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("sender", sender_addr)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("burn_amount", burn_amount)
        .add_attribute("commission_amount", commission_amount))
}
