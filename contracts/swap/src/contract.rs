#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_binary, Uint128, BankMsg, coins, Addr, CosmosMsg, WasmMsg};
use cw2::set_contract_version;
use cw20_legacy::msg::ExecuteMsg::Transfer;
use shared::querier::{query_balance, query_price, query_token_balance};
use crate::ContractError::{InsufficientContractBalance, InvalidQuantity, Unauthorized};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
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
        owner: info.sender,
        token_address: msg.token_address,
        oracle_address: msg.oracle_address,
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy { } => try_buy(deps, env, info),
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, env, info, Uint128::from(amount))
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryBalance { } => to_binary(&query_balance(&deps.querier, &env.contract.address, String::from("uluna"))?),
        QueryMsg::QueryTokenAddress {} => to_binary(&query_token_address(deps)?)
    }
}

pub fn try_buy(
    deps: DepsMut,
    env: Env,
    info: MessageInfo
) -> Result<Response, ContractError> {
    let mut luna_value = Uint128::from(0u64);
    for coin in info.funds {
        if coin.denom == "uluna" {
            if coin.amount <= Uint128::from(0u64) {
                return Err(InvalidQuantity);
            }
            else {
                luna_value = coin.amount;
            }
        }
    };
    if luna_value == Uint128::from(0u64) {
        return Err(InvalidQuantity);
    }
    let state = STATE.load(deps.storage)?;
    let price = query_price(&deps.querier, &state.oracle_address)?;
    let saku_value: Uint128 = luna_value / price;
    let contract_balance = query_token_balance(&deps.querier, &state.token_address, &env.contract.address)?;
    if contract_balance < saku_value {
        return Err(InsufficientContractBalance);
    }
    let msg = Transfer {
        recipient: String::from(info.sender),
        amount: saku_value
    };
    let res = WasmMsg::Execute {
        contract_addr: String::from(state.token_address),
        msg: to_binary(&msg)?,
        funds: coins(0, "uluna"),
    };
    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(res))
        .add_attribute("method", "buy"))
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(Unauthorized {});
    }
    let contract_balance = query_balance(&deps.querier, &env.contract.address, String::from("uluna"))?;
    if contract_balance < amount {
        return Err(InsufficientContractBalance);
    }
    let msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(amount.u128(), "uluna")
    };
    Ok(Response::new()
        .add_message(CosmosMsg::Bank(msg))
        .add_attribute("method", "withdraw"))
}

pub fn query_token_address(deps: Deps) -> StdResult<Addr> {
    let state = STATE.load(deps.storage)?;
    Ok(state.token_address)
}


#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, coins, from_binary, Uint128};
    use crate::contract::{instantiate, query};
    use crate::msg::{InstantiateMsg, QueryMsg};
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg { token_address: Addr::unchecked("cw20"), oracle_address: Addr::unchecked("oracle") };
        let info = mock_info("creator", &coins(100000, "uluna"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryBalance {}).unwrap();
        let value: Uint128 = from_binary(&res).unwrap();
        // assert_eq!(res, Err(StdError::generic_err("not implemented")));
        assert_eq!(Uint128::from(0u64), value);
        let res2 = query(deps.as_ref(), mock_env(), QueryMsg::QueryTokenAddress {}).unwrap();
        let addr: Addr = from_binary(&res2).unwrap();
        assert_eq!(addr.to_string(), "cw20");

        let a = Uint128::from(100000u64);
        let b = Uint128::from(110u64);
        let c: Uint128 = a / b;
        println!("{}", c)
    }
}
