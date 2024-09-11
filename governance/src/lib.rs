use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use crate::state::{PROPOSALS};

pub fn submit_proposal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String,
    description: String,
) -> Result<Response, ContractError> {
    let proposer = info.sender.clone();

    PROPOSALS.save(deps.storage, title.clone(), &Proposal {
        title,
        description,
        proposer: proposer.clone(),
        votes_for: Uint128::zero(),
        votes_against: Uint128::zero(),
    })?;

    Ok(Response::new()
        .add_attribute("action", "submit_proposal")
        .add_attribute("proposer", proposer))
}
