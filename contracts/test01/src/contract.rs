use crate::error::ContractResponse;
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::query::query_config;
use crate::state::instantiate_contract;
use classic_bindings::TerraQuery;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut<TerraQuery>,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    instantiate_contract(deps.into_empty(), env, msg)
}

#[entry_point]
pub fn execute(
    _deps: DepsMut<TerraQuery>,
    _env: Env,
    _info: MessageInfo,
    _: Empty,
) -> ContractResponse {
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps<TerraQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let deps_empty = deps.into_empty();
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps_empty)?),
    }
}
