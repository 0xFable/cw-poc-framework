use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::{App};
use anyhow::Result as AnyResult;

// A Driver is a trait which other implementors must implement to achieve a common schema across Drivers
pub trait Driver {
    fn init_contracts(router: &mut App, owner: &Addr) -> Self;
}

// A DexDriver is a trait which includes common operations which usually have similar inputs 
pub trait DexDriver {
    fn deposit_liquidity(&self, app: &mut App, pair_instance: &Addr, token_x_instance: &Addr, token_x_amount: Uint128, token_y_instance: &Addr, token_y_amount: Uint128, caller: &Option<Addr>);
    fn withdraw_liquidity(&self, app: &mut App, pair_instance: &Addr, lp_token_instance: &Addr, lp_token_amount: Uint128, caller: &Option<Addr>);
    fn create_pool(&self, app: &mut App, token1: &Addr, token2: &Addr) -> AnyResult<Addr>;
    fn init_cw20_token(&self, app: &mut App, name: &str) -> AnyResult<Addr>;
}

