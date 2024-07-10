use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub cw20_token: String,
    pub owner: String,               // indirizzo proprietario del contratto
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetResponse<Config>)]
    GetConfig {},
}

#[cw_serde]
pub struct GetResponse<T> {
    pub value: T,
}
