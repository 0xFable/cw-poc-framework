// A Driver is a trait which other implementors must implement to achieve a common schema across Drivers
pub trait Driver {
    pub fn init_contracts(router: &mut App, owner: Addr) -> App
}