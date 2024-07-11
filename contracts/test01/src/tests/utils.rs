use crate::contract::{execute, instantiate, query};
use crate::msg::InstantiateMsg;
use classic_bindings::TerraQuery;
use cosmwasm_std::{Addr, Empty, Uint128};
use cw_multi_test::{custom_app, BasicApp, Contract, ContractWrapper, Executor};

const GOVERNANCE: &str = "governance";
const TERRA: &str = "TERRA";

pub type MyApp = BasicApp<Empty, TerraQuery>;

pub fn my_app() -> MyApp {
    custom_app::<Empty, TerraQuery, _>(|_, _, _| {})
}

/// Convenient utility function for creating addresses in tests.
pub fn addr(input: &str) -> Addr {
    Addr::unchecked(input)
}

/// Convenient utility function for creating governance address in tests.
pub fn governance_addr() -> Addr {
    addr(GOVERNANCE)
}

pub fn init_contracts(app: &mut MyApp) -> (Addr, Addr) {
    let token_contract_addr = init_token_contract(app);
    let test01_contract_addr = init_test01_contract(app, &token_contract_addr);
    (test01_contract_addr, token_contract_addr)
}

pub fn init_test01_contract(app: &mut MyApp, token_contract_addr: &Addr) -> Addr {
    let governance_addr = addr(GOVERNANCE);

    let contract = ContractWrapper::new(execute, instantiate, query);
    let contract: Box<(dyn Contract<Empty, TerraQuery> + 'static)> = Box::new(contract);

    let code_id = app.store_code(contract);

    app.instantiate_contract(
        code_id,
        governance_addr.clone(),
        &InstantiateMsg {
            owner: governance_addr.to_string(),
            cw20_token: token_contract_addr.to_string(),
        },
        &[],
        "staking",
        None,
    )
    .unwrap()
}

pub fn init_token_contract(app: &mut MyApp) -> Addr {
    let governance_addr = addr(GOVERNANCE);

    let contract = Box::new(ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));

    let code_id = app.store_code(contract);

    let instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: "cw20 token".to_string(),
        symbol: TERRA.to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(cw20::MinterResponse {
            minter: governance_addr.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    app.instantiate_contract(
        code_id,
        governance_addr.clone(),
        &instantiate_msg,
        &[],
        TERRA.to_string(),
        None,
    )
    .unwrap()
}

pub fn mint_tokens(
    app: &mut MyApp,
    owner: &Addr,
    token_contract_addr: &Addr,
    amount: u128,
    recipient: &Addr,
) {
    let msg = cw20::Cw20ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount: Uint128::new(amount),
    };
    app.execute_contract(owner.clone(), token_contract_addr.clone(), &msg, &[])
        .unwrap();
}

pub fn query_tokens(app: &MyApp, token_contract_addr: &Addr, address: &Addr) -> u128 {
    let msg = cw20::Cw20QueryMsg::Balance {
        address: address.to_string(),
    };
    let response: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(token_contract_addr, &msg)
        .unwrap();
    response.balance.u128()
}
