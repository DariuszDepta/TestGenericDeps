use crate::tests::utils::{addr, governance_addr, init_token_contract, mint_some_tokens, my_app};

#[test]
fn minting_come_tokens_should_work() {
    // create my chain
    let mut app = my_app();

    // prepare governance and recipient addresses
    let governance_addr = governance_addr();
    let recipient_addr = addr("recipient");

    // instantiate token contract and get its address
    let token_contract_addr = init_token_contract(&mut app);

    // mint some tokens for the recipient
    mint_some_tokens(
        &mut app,
        &governance_addr,
        &token_contract_addr,
        100,
        &recipient_addr,
    );
}
