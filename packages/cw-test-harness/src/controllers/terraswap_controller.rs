
use crate::common_integration::{store_pair_code, store_factory_code, store_token_code, AvailablePlatforms};
use crate::traits::traits::{Driver, DexDriver};
use anyhow::Result as AnyResult;
use cosmwasm_std::{to_binary, Addr, Uint128};
use cw_multi_test::{App};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_multi_test::Executor;
use terraswap::asset::PairInfo;
use terraswap::asset::{Asset, AssetInfo};

use cw20::{Cw20ExecuteMsg};

/// AstroportController is a wrapper around a series of Addrs which together form 'Astroport'
/// Together, when imported AstroportController provides a drop-in suite for initialising a set of astroport 
/// contracts into your testing environment and then performing actions on them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TerraSwapController(pub Addr, pub u64, pub Addr, pub u64);

/// Implement the Driver trait which enforces certain method declarations to ensure a common Controller schema
impl Driver for TerraSwapController{
    fn init_contracts(app: &mut App, owner: &Addr) -> Self {
        let pair_code_id = store_pair_code(app, Some(AvailablePlatforms::TERRASWAP));
        let factory_code_id = store_factory_code(app, Some(AvailablePlatforms::TERRASWAP));
        let tswap_token_variant_id = store_token_code(app, Some(AvailablePlatforms::TERRASWAP));

        let msg = terraswap::factory::InstantiateMsg {
            pair_code_id: pair_code_id,
            token_code_id: tswap_token_variant_id
        };

        let factory = app
            .instantiate_contract(factory_code_id, owner.clone(), &msg, &[], "Factory", None)
            .unwrap();

        // TODO: Update returns 
        Self {
           0: owner.clone(),
           1: pair_code_id,
           2: factory,
           3: tswap_token_variant_id,
        }
    }
}



impl DexDriver for TerraSwapController{
    fn init_cw20_token(&self, app: &mut App, name: &str) -> AnyResult<Addr> {
        let msg = astroport::token::InstantiateMsg {
            name: name.to_string(),
            symbol: name.to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: None,
            marketing: None,
        };

        app.instantiate_contract(
            self.3,
            Addr::unchecked(self.0.clone()),
            &msg,
            &[],
            name.to_string(),
            None,
        )
    }

    fn create_pool(&self, app: &mut App, token1: &Addr, token2: &Addr) -> AnyResult<Addr> {
        let asset_infos = [
            AssetInfo::Token {
                contract_addr: token1.to_string(),
            },
            AssetInfo::Token {
                contract_addr: token2.to_string(),
            },
        ];

        app.execute_contract(
            Addr::unchecked(self.0.clone()),
            self.2.clone(),
            &terraswap::factory::ExecuteMsg::CreatePair {
                asset_infos: asset_infos.clone(),
            },
            &[],
        )?;

        let res: PairInfo = app.wrap().query_wasm_smart(
            self.2.clone(),
            &terraswap::factory::QueryMsg::Pair { asset_infos },
        )?;

        Ok(Addr::unchecked(res.liquidity_token))
    }

    fn deposit_liquidity(&self, app: &mut App, pair_instance: &Addr, token_x_instance: &Addr, token_x_amount: Uint128, token_y_instance: &Addr, token_y_amount: Uint128, caller: &Option<Addr>){
        // TODO: Add Allowance Checks 
        // TODO: Handle native vs cw20 variants, rn this is cw20 ONLY
        let msg = terraswap::pair::ExecuteMsg::ProvideLiquidity {
            assets: [
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_x_instance.to_string(),
                    },
                    amount: token_x_amount,
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_y_instance.to_string(),
                    },
                    amount: token_y_amount,
                },
            ],
            slippage_tolerance: None,
            receiver: None

        };
        app.execute_contract(caller.clone().unwrap_or(self.0.clone()), pair_instance.clone(), &msg, &[])
        .unwrap();
    }
    fn withdraw_liquidity(&self, app: &mut App, pair_instance: &Addr, lp_token_instance: &Addr, lp_token_amount: Uint128, caller: &Option<Addr>){
        // TODO: Add Allowance Checks 
        // TODO: Handle native vs cw20 variants, rn this is cw20 ONLY
        let msg = Cw20ExecuteMsg::Send {
            contract: pair_instance.to_string(),
            amount: lp_token_amount,
            msg: to_binary(&terraswap::pair::Cw20HookMsg::WithdrawLiquidity {}).unwrap(),
        };
        app
        .execute_contract(caller.clone().unwrap_or(self.0.clone()), lp_token_instance.clone(), &msg, &[])
        .unwrap();
    }
}

impl TerraSwapController {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

}