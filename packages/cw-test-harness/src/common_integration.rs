

pub fn contract_cw20_token() -> Box<dyn Contract<Empty>> {
    // Instantiate cw20 Token Contract
    let whale_token_contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(whale_token_contract)
}

pub fn mock_app() -> App {
    let mut app = App::default();
    app
}