#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Uint128};
use cw2::set_contract_version;
use crate::ContractError::Unauthorized;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use shared::oracle::PriceResponse;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        price: msg.price,
        owner: info.sender.clone()
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("init_price", msg.price.to_string())
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdatePrice { price } => try_update_price(deps, info, price)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryPrice { } => to_binary(&query_price(deps)?)
    }
}

pub fn try_update_price(
    deps: DepsMut,
    info: MessageInfo,
    price: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.price = price;
        Ok(state)
    })?;
    Ok(Response::new()
        .add_attribute("method", "update_price")
        .add_attribute("new_price", price.to_string()))
}

pub fn query_price(deps: Deps) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    let res = PriceResponse {
        price: state.price
    };
    Ok(res)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg { price: Uint128::from(17u64) };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap();
        let value: PriceResponse = from_binary(&res).unwrap();
        // assert_eq!(res, Err(StdError::generic_err("not implemented")));
        assert_eq!(Uint128::from(17u64), value.price);
    }

    #[test]
    fn increment() {}
}
