#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult };
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, GetFundResponse, InstantiateMsg, QueryMsg };
use crate::state::{ State, STATE };

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:jackpot";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    let state = State {
        fund: msg.fund,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(
        Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("owner", info.sender)
            .add_attribute("fund", msg.fund.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddFunds { fund } => execute::add_funds(deps, info, fund),
    }
}

pub mod execute {
    use super::*;

    pub fn add_funds(
        _deps: DepsMut,
        _info: MessageInfo,
        fund: i32
    ) -> Result<Response, ContractError> {
        STATE.update(
            _deps.storage,
            |mut state| -> Result<_, ContractError> {
                if _info.sender != state.owner {
                    return Err(ContractError::Unauthorized {});
                }
                state.fund = fund;
                Ok(state)
            }
        )?;
        Ok(Response::new().add_attribute("action", "add_funds"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFunds {} => to_json_binary(&query::get_funds(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_funds(deps: Deps) -> StdResult<GetFundResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetFundResponse { fund: state.fund })
    }
}
