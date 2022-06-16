use cosmwasm_std::{Addr};
use cw_multi_test::{AppBuilder, App};
use cw_test_harness::controllers::{TerraSwapController, AstroportController};

pub struct SimpleScenario {
    pub app: App,
    pub owner: Addr,
    pub alice: Addr,
    pub bob: Addr,
    pub terraswap: TerraSwapController,
    pub astroport: AstroportController,

}

impl SimpleScenario {
    pub fn new() -> Self {
        let mut app = AppBuilder::new().build(|_app, _, _storage| {});

        let owner = Addr::unchecked("owner");
        let alice = Addr::unchecked("alice");
        let bob = Addr::unchecked("bob");
        // TODO: Improve this experience, can we get away without passing things? 
        let astroport = AstroportController(Addr::unchecked(""),Default::default(),Addr::unchecked(""),Default::default());
        let terraswap = TerraSwapController(Addr::unchecked(""),Default::default(),Addr::unchecked(""),Default::default());

        // We now have 3 'users' and a bunch of contracts instantiated for our interactions against both TerraSwap and Astroport
        Self {
            app,
            owner,
            alice,
            bob,
            terraswap,
            astroport
        }
    }
}
