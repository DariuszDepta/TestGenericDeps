use super::utils::*;

#[test]
fn instantiate_check_config() {
    let mut app = my_app();
    let (test01_contract_addr, token_contract_addr) = init_contracts(&mut app);

    // TODO Remove the following lines when contract addresses are used somewhere else.
    assert_eq!("contract0", token_contract_addr.as_str());
    assert_eq!("contract1", test01_contract_addr.as_str());
}

#[test]
fn instantiating_token_contract_should_work() {
    let mut app = my_app();
    let token_contract_addr = init_token_contract(&mut app);
    assert_eq!("contract0", token_contract_addr.as_str());
}

#[test]
fn instantiating_test01_contract_should_work() {
    let mut app = my_app();
    let token_contract_addr = init_token_contract(&mut app);
    let test01_contract_addr = init_test01_contract(&mut app, &token_contract_addr);
    assert_eq!("contract1", test01_contract_addr.as_str());
}
