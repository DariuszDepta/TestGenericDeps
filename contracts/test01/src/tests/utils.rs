use classic_bindings::TerraQuery;
use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{attr, Addr, Empty, Uint128};
use cw_multi_test::{App, AppBuilder, BankKeeper, Contract, ContractWrapper, Executor};

use crate::contract::{execute, instantiate, query};
use crate::msg::InstantiateMsg;

pub fn mock_app() -> App {
    let api = MockApi::default();
    let env = mock_env();
    let bank = BankKeeper::new();
    let storage = MockStorage::new();

    let app = AppBuilder::new()
        .with_api(api)
        .with_block(env.block)
        .with_bank(bank)
        .with_storage(storage)
        .build(|_, _, _| {});
    app
}

pub fn init_contracts(app: &mut App) -> (Addr, Addr) {
    let governance = Addr::unchecked("governance");

    let contract = Box::new(ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    ));

    let code_id = app.store_code(contract);

    let instantiate_msg = cw20_base::msg::InstantiateMsg {
        name: String::from("cw20 token"),
        symbol: String::from("TERRA"),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(cw20::MinterResponse {
            minter: governance.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    let cw20_token_contract_addr = app
        .instantiate_contract(
            code_id,
            governance.clone(),
            &instantiate_msg,
            &[],
            String::from("TERRA"),
            None,
        )
        .unwrap();

    let contract = ContractWrapper::new(execute, instantiate, query);
    let contract: Box<(dyn Contract<Empty, TerraQuery> + 'static)> = Box::new(contract);

    //// OMG HOW TO DO IT?????????????
    let code_id = app.store_code(contract);

    let test01_contract_addr = app
        .instantiate_contract(
            code_id,
            governance,
            &InstantiateMsg {
                owner: governance.clone().to_string(),
                cw20_token: cw20_token_contract_addr.to_string(),
            },
            &[],
            "staking",
            None,
        )
        .unwrap();

    (test01_contract_addr, cw20_token_contract_addr)
}

pub fn mint_some_token(
    app: &mut App,
    owner: Addr,
    token_contract_addr: Addr,
    amount: Uint128,
    to: String,
) {
    let msg = cw20::Cw20ExecuteMsg::Mint {
        recipient: to.clone(),
        amount: amount,
    };
    let res = app
        .execute_contract(owner.clone(), token_contract_addr.clone(), &msg, &[])
        .unwrap();
    assert_eq!(res.events[1].attributes[1], attr("action", "mint"));
    assert_eq!(res.events[1].attributes[2], attr("to", to));
    assert_eq!(res.events[1].attributes[3], attr("amount", amount));
}
