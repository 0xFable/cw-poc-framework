
use crate::common_integration::{store_pair_code, store_factory_code, store_token_code, AvailablePlatforms};
use crate::driver_abstraction::{Driver};
use cosmwasm_std::{Addr};
use cw_multi_test::{App};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/// AstroportController is a wrapper around a series of Addrs which together form 'Astroport'
/// Together, when imported AstroportController provides a drop-in suite for initialising a set of astroport 
/// contracts into your testing environment and then performing actions on them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AstroportController(pub Addr);

/// Implement the Driver trait which enforces certain method declarations to ensure a common Controller schema
impl Driver for AstroportController{
    fn init_contracts(app: &mut App, owner: &Addr) -> Self {
        let pair_code_id = store_pair_code(app, Some(AvailablePlatforms::ASTROPORT));
        let factory_code_id = store_factory_code(app, Some(AvailablePlatforms::ASTROPORT));
        let astro_token_variant_id = store_token_code(app, Some(AvailablePlatforms::ASTROPORT));

        // TODO: Update returns 
        Self {
           0: owner.clone()
        }
    }
}

impl AstroportController {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

}