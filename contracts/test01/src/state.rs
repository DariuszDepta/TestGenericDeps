use crate::msg::InstantiateMsg;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, Env, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub cw20_token_contract: Addr,
    pub owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate_contract(deps: DepsMut, _env: Env, msg: InstantiateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let store = deps.storage;
    let owner: Addr = deps.api.addr_validate(&msg.owner)?;
    let cw20_token_contract: Addr = deps.api.addr_validate(&msg.cw20_token)?;
    let config: Config = Config {
        owner,
        cw20_token_contract,
    };
    CONFIG.save(store, &config)?;
    Ok(Response::new())
}
