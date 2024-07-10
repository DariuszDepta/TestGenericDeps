use super::utils::{init_contracts, mock_app};

#[test]
fn instantiate_check_config() {
    let app = &mut mock_app();
    let (test01_contract_addr, cw20_contract_addr) = init_contracts(app);

    // TODO check query config
}
