use cw_multi_test::{next_block, Contract, App, ContractWrapper};
use cosmwasm_std::{Empty};

use cw20_base;
use terraswap_token;
use terraswap_pair;
use terraswap_factory;
use astroport_factory;
use astroport_pair;
use astroport_token;

// TODO: Move elsewhere
pub enum AvailablePlatforms{
    ASTROPORT,
    TERRASWAP
}

pub fn contract_cw20_token() -> Box<dyn Contract<Empty>> {
    // Instantiate cw20 Token Contract
    let whale_token_contract = ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(whale_token_contract)
}

pub fn store_token_code(app: &mut App, platform: Option<AvailablePlatforms>) -> u64 {
    let wrapped_token_contract;
    match platform.unwrap_or(AvailablePlatforms::ASTROPORT) {
        
        AvailablePlatforms::ASTROPORT => {
         wrapped_token_contract = Box::new(ContractWrapper::new_with_empty(
            astroport_token::contract::execute,
            astroport_token::contract::instantiate,
            astroport_token::contract::query,
        ));
        }
        AvailablePlatforms::TERRASWAP => {
            wrapped_token_contract = Box::new(ContractWrapper::new_with_empty(
            terraswap_token::contract::execute,
            terraswap_token::contract::instantiate,
            terraswap_token::contract::query,
        ));
        }
        (_) => panic!("Unsupported platform")
    }

    app.store_code(wrapped_token_contract)
}

pub fn store_pair_code(app: &mut App, platform: Option<AvailablePlatforms>) -> u64 {
    let pair_contract: Box<dyn Contract<Empty>>;
    match platform.unwrap_or(AvailablePlatforms::ASTROPORT) {
        
        AvailablePlatforms::ASTROPORT => {
            pair_contract = Box::new(ContractWrapper::new_with_empty(
                astroport_pair::contract::execute,
                astroport_pair::contract::instantiate,
                astroport_pair::contract::query,
            ));
            }
        AvailablePlatforms::TERRASWAP => {
            pair_contract = Box::new(ContractWrapper::new_with_empty(
                terraswap_pair::contract::execute,
                terraswap_pair::contract::instantiate,
                terraswap_pair::contract::query,
            ));
            }
        (_) => panic!("Unsupported platform")
    }
    
    app.store_code(pair_contract)
}

#[allow(dead_code)]
fn store_factory_code(app: &mut App, platform: Option<AvailablePlatforms>) -> u64 {
    let factory_contract: Box<dyn Contract<Empty>>;

    match platform.unwrap_or(AvailablePlatforms::ASTROPORT) {
        
        AvailablePlatforms::ASTROPORT => {
            factory_contract = Box::new(
                ContractWrapper::new(
                    astroport_factory::contract::execute,
                    astroport_factory::contract::instantiate,
                    astroport_factory::contract::query,
                )
                .with_reply(astroport_factory::contract::reply),
            );
        }
        AvailablePlatforms::TERRASWAP => {
            factory_contract = Box::new(
                ContractWrapper::new(
                    terraswap_factory::contract::execute,
                    terraswap_factory::contract::instantiate,
                    terraswap_factory::contract::query,
                )
                .with_reply(terraswap_factory::contract::reply),
            );
        }
        (_) => panic!("Unsupported platform")
    }
    
    app.store_code(factory_contract)
}



pub fn mock_app() -> App {
    let app = App::default();
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