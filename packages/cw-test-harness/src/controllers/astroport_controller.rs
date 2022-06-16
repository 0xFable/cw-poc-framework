
use crate::common_integration::{store_pair_code, store_factory_code, store_token_code, AvailablePlatforms};
use crate::traits::traits::{Driver, DexDriver};
use anyhow::Result as AnyResult;
use cosmwasm_std::{to_binary, Addr, Uint128};
use cw_multi_test::{App};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_multi_test::Executor;
use astroport::asset::PairInfo;
use astroport::asset::{Asset, AssetInfo};
use astroport::factory::{PairConfig, PairType};
use astroport::pair::{
    ConfigResponse, CumulativePricesResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg,
    TWAP_PRECISION
};
use cw20::{BalanceResponse, Cw20Coin, Cw20ExecuteMsg, Cw20QueryMsg, MinterResponse};

/// AstroportController is a wrapper around a series of Addrs which together form 'Astroport'
/// Together, when imported AstroportController provides a drop-in suite for initialising a set of astroport 
/// contracts into your testing environment and then performing actions on them.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AstroportController(pub Addr, pub u64, pub Addr, pub u64);

/// Implement the Driver trait which enforces certain method declarations to ensure a common Controller schema
impl Driver for AstroportController{
    fn init_contracts(app: &mut App, owner: &Addr) -> Self {
        let pair_code_id = store_pair_code(app, Some(AvailablePlatforms::ASTROPORT));
        let factory_code_id = store_factory_code(app, Some(AvailablePlatforms::ASTROPORT));
        let astro_token_variant_id = store_token_code(app, Some(AvailablePlatforms::ASTROPORT));

        let msg = astroport::factory::InstantiateMsg {
            pair_configs: vec![PairConfig {
                code_id: pair_code_id,
                pair_type: PairType::Xyk {},
                total_fee_bps: 100,
                maker_fee_bps: 10,
                is_disabled: false,
                is_generator_disabled: false,
            }],
            token_code_id: astro_token_variant_id,
            fee_address: None,
            generator_address: None,
            owner: owner.to_string(),
            whitelist_code_id: 0,
        };

        let factory = app
            .instantiate_contract(factory_code_id, owner.clone(), &msg, &[], "Factory", None)
            .unwrap();

        // TODO: Update returns 
        Self {
           0: owner.clone(),
           1: pair_code_id,
           2: factory,
           3: astro_token_variant_id,
        }
    }
}



impl DexDriver for AstroportController{
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
                contract_addr: token1.clone(),
            },
            AssetInfo::Token {
                contract_addr: token2.clone(),
            },
        ];

        app.execute_contract(
            Addr::unchecked(self.0.clone()),
            self.2.clone(),
            &astroport::factory::ExecuteMsg::CreatePair {
                pair_type: PairType::Xyk {},
                asset_infos: asset_infos.clone(),
                init_params: None,
            },
            &[],
        )?;

        let res: PairInfo = app.wrap().query_wasm_smart(
            self.2.clone(),
            &astroport::factory::QueryMsg::Pair { asset_infos },
        )?;

        Ok(res.liquidity_token)
    }

    fn deposit_liquidity(&self, app: &mut App, pair_instance: &Addr, token_x_instance: &Addr, token_x_amount: Uint128, token_y_instance: &Addr, token_y_amount: Uint128, caller: &Option<Addr>){
        // TODO: Add Allowance Checks 
        // TODO: Handle native vs cw20 variants, rn this is cw20 ONLY
        let msg = ExecuteMsg::ProvideLiquidity {
            assets: [
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_x_instance.clone(),
                    },
                    amount: token_x_amount,
                },
                Asset {
                    info: AssetInfo::Token {
                        contract_addr: token_y_instance.clone(),
                    },
                    amount: token_y_amount,
                },
            ],
            slippage_tolerance: None,
            auto_stake: None,
            receiver: None,
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
            msg: to_binary(&Cw20HookMsg::WithdrawLiquidity {}).unwrap(),
        };
        app
        .execute_contract(caller.clone().unwrap_or(self.0.clone()), lp_token_instance.clone(), &msg, &[])
        .unwrap();
    }
}

impl AstroportController {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

}