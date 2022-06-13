use cw_multi_test::{next_block, Contract, App, ContractWrapper};
use cosmwasm_std::{Empty};

pub fn contract_cw20_token() -> Box<dyn Contract<Empty>> {
    // Instantiate cw20 Token Contract
    let whale_token_contract = ContractWrapper::new_with_empty(
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

pub fn advance_one_block(mut app: App) -> App {
    // Consider deprecating as there is next_block from use cw_multi_test::{next_block}
    app.update_block(|block| {
        block.height += 1;
        block.time = block.time.plus_seconds(5);
    });
    app
}

pub fn advance_n_blocks(mut app: App, blocks_to_advance: u64, block_time_interval: Option<u64>) -> App {
    app.update_block(|block| {
        block.height += blocks_to_advance;
        // Advance time 
        block.time = block.time.plus_seconds(blocks_to_advance * block_time_interval.unwrap_or(5));
    });
    app
}