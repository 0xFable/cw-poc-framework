use cosmwasm_std::{Addr};
use cw_multi_test::{App};
// A Driver is a trait which other implementors must implement to achieve a common schema across Drivers
pub trait Driver {
    fn init_contracts(router: &mut App, owner: &Addr) -> Self;
}