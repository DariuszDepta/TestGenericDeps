use cosmwasm_std::{Deps, StdResult};

use crate::msg::GetResponse;
use crate::state::{Config, CONFIG};

pub fn query_config(deps: Deps) -> StdResult<GetResponse<Config>> {
    let store = deps.storage;
    let config: Config = CONFIG.load(store)?;
    Ok(GetResponse { value: config })
}
