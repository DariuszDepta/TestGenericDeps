use crate::tests::utils::{
    addr, governance_addr, init_token_contract, mint_tokens, my_app, query_tokens,
};

#[test]
fn minting_come_tokens_should_work() {
    // amount of tokens to be minted
    const AMOUNT: u128 = 100;

    // create my chain
    let mut app = my_app();

    // prepare governance and recipient addresses
    let governance_addr = governance_addr();
    let recipient_addr = addr("recipient");

    // instantiate token contract and get its address
    let token_contract_addr = init_token_contract(&mut app);

    // mint some tokens for the recipient
    mint_tokens(
        &mut app,
        &governance_addr,
        &token_contract_addr,
        AMOUNT,
        &recipient_addr,
    );

    // make sure that the recipient has the expected amount of tokens
    assert_eq!(
        AMOUNT,
        query_tokens(&app, &token_contract_addr, &recipient_addr)
    );
}
