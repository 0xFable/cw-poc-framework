use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    from_binary, from_slice, to_binary, Addr, Api, Binary, Coin, ContractResult, Decimal,
    Empty, OwnedDeps, Querier, QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, WasmQuery,
};

use astroport::asset::{Asset, AssetInfo, PairInfo};
use astroport::factory::PairType;
use astroport::pair::PoolResponse;
use cosmwasm_storage::to_length_prefixed;
use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg};
use std::collections::HashMap;
use terraswap::asset::{AssetInfoRaw, PairInfoRaw};


/// mock_dependencies is a drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// this uses our CustomQuerier.
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
        custom_query_type: Default::default(),
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<Empty>,
    terraswap_pair_querier: TerraswapPairQuerier,
    token_querier: TokenQuerier,
    astroport_factory_querier: AstroportFactoryQuerier,
}

#[derive(Clone, Default)]
pub struct AstroportFactoryQuerier {
    pairs: HashMap<String, PairInfo>,
}

impl AstroportFactoryQuerier {
    pub fn new(pairs: &[(&String, &PairInfo)]) -> Self {
        AstroportFactoryQuerier {
            pairs: pairs_to_map(pairs),
        }
    }
}

pub(crate) fn pairs_to_map_astro(pairs: &[(&String, &PairInfo)]) -> HashMap<String, PairInfo> {
    let mut pairs_map: HashMap<String, PairInfo> = HashMap::new();
    for (key, pair) in pairs.iter() {
        pairs_map.insert(key.to_string(), (*pair).clone());
    }
    pairs_map
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    // this lets us iterate over all pairs that match the first string
    balances: HashMap<String, HashMap<String, Uint128>>,
}

impl TokenQuerier {
    pub fn new(balances: &[(&String, &[(&String, &Uint128)])]) -> Self {
        TokenQuerier {
            balances: balances_to_map(balances),
        }
    }
}

pub(crate) fn balances_to_map(
    balances: &[(&String, &[(&String, &Uint128)])],
) -> HashMap<String, HashMap<String, Uint128>> {
    let mut balances_map: HashMap<String, HashMap<String, Uint128>> = HashMap::new();
    for (contract_addr, balances) in balances.iter() {
        let mut contract_balances_map: HashMap<String, Uint128> = HashMap::new();
        for (addr, balance) in balances.iter() {
            contract_balances_map.insert(addr.to_string(), **balance);
        }

        balances_map.insert(contract_addr.to_string(), contract_balances_map);
    }
    balances_map
}


#[derive(Clone, Default)]
pub struct TerraswapPairQuerier {
    pairs: HashMap<String, PairInfo>,
}

impl TerraswapPairQuerier {
    pub fn new(pairs: &[(&String, &PairInfo)]) -> Self {
        TerraswapPairQuerier {
            pairs: pairs_to_map(pairs),
        }
    }
}

pub(crate) fn pairs_to_map(pairs: &[(&String, &PairInfo)]) -> HashMap<String, PairInfo> {
    let mut pairs_map: HashMap<String, PairInfo> = HashMap::new();
    for (key, pair) in pairs.iter() {
        pairs_map.insert(key.to_string(), (*pair).clone());
    }
    pairs_map
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        println!("Request hit the mock querier \n {:?}", request);
        match &request {
            
            // Manual mocking for smart queries
            // Here we can do alot to mock out messages either by defining a new
            // MockQueryMsg with each call as a type of it
            // Or for more quick multi-contract mocking consider using the contract_addr
            // or directly parsing the message if it is unique
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                // Handle calls for Example Contract anchor
                if contract_addr == &String::from("anchor_mm") {
                    // Handle Anchor EpochStateQuery
                    if msg == &Binary::from(r#"{"epoch_state":{}}"#.as_bytes()) {
                        return SystemResult::Ok(ContractResult::from(to_binary(
                            // &mock_epoch_state(),
                            "Mocked the epoch state call"
                        )));
                    }

                    return SystemResult::Ok(ContractResult::from(to_binary(
                        &Cw20BalanceResponse {
                            balance: Uint128::zero(),
                        },
                    )));
                }

               

                // Handle calls for contract which represents an lp token
                // TODO: Right now msgs are handled via IFs
                // Once we have close to the full set of command mocked out(or half) we should
                // switch this to a match if possible
                if contract_addr == &String::from("astro_based_lp_token") {
                    print!(
                        "Handling call for astro LP token with name {:?}",
                        contract_addr
                    );

                    // Handle Pool query
                    if msg == &Binary::from(r#"{"pool":{}}"#.as_bytes()) {
                        let msg_pool = PoolResponse {
                            assets: [
                                Asset {
                                    amount: Uint128::from(10000u128),
                                    info: AssetInfo::NativeToken {
                                        denom: "whale".to_string(),
                                    },
                                },
                                Asset {
                                    amount: Uint128::from(10000u128),
                                    info: AssetInfo::NativeToken {
                                        denom: "uusd".to_string(),
                                    },
                                },
                            ],
                            total_share: Uint128::from(1000u128),
                        };
                        return SystemResult::Ok(ContractResult::from(to_binary(&msg_pool)));
                    }

                    // Handle pair info
                    // if msg == &Binary::from(r#"{"pair":{}}"#.as_bytes()) {
                    else {
                        println!("Were looking for pair");
                        let msg_balance = PairInfo {
                            asset_infos: [
                                AssetInfo::Token {
                                    contract_addr: Addr::unchecked("bluna"),
                                },
                                AssetInfo::NativeToken {
                                    denom: "uusd".to_string(),
                                },
                            ],
                            contract_addr: Addr::unchecked("PAIR0000"),
                            liquidity_token: Addr::unchecked("liqtoken"),
                            pair_type: PairType::Xyk {},
                        };
                        return SystemResult::Ok(ContractResult::from(to_binary(&msg_balance)));
                    }
                } else {
                    // From here we are starting to deal with CW20 type tokens, try to match expected types and messages
                    match from_binary(msg).unwrap() {
                        Cw20QueryMsg::Balance { address } => {
                            if contract_addr
                                == &String::from("exceptional_token_without_normal_balance")
                            {
                                return SystemResult::Ok(ContractResult::Ok(
                                    to_binary(&Cw20BalanceResponse {
                                        balance: Uint128::zero(),
                                    })
                                    .unwrap(),
                                ));
                            }

                            let balances: &HashMap<String, Uint128> =
                                match self.token_querier.balances.get(contract_addr) {
                                    Some(balances) => balances,
                                    None => {
                                        return SystemResult::Err(SystemError::InvalidRequest {
                                            error: format!(
                                                "No balance info exists for the contract {}",
                                                contract_addr
                                            ),
                                            request: msg.as_slice().into(),
                                        });
                                    }
                                };

                            let balance = match balances.get(&address) {
                                Some(v) => *v,
                                None => {
                                    return SystemResult::Ok(ContractResult::Ok(
                                        to_binary(&Cw20BalanceResponse {
                                            balance: Uint128::zero(),
                                        })
                                        .unwrap(),
                                    ));
                                }
                            };

                            SystemResult::Ok(ContractResult::Ok(
                                to_binary(&Cw20BalanceResponse { balance }).unwrap(),
                            ))
                        }
                        _ => panic!("DO NOT ENTER HERE"),
                    }
                }
            }
            // Handle Raw Queries, these are cheaper queries than Smart variants and generally serve specific purposes
            // for contracts
            QueryRequest::Wasm(WasmQuery::Raw { contract_addr, key }) => {
                // Handle Dex Variants, they typically use raw queries for certain ops.
                let key: &[u8] = key.as_slice();
                let prefix_pair_info = to_length_prefixed(b"pair_info").to_vec();

                // Exceptional case - handle a user-defined key value registry
                if contract_addr == &String::from("registry") {
                    return SystemResult::Ok(ContractResult::Ok(
                        to_binary(&"mocked_registry_value".to_string()).unwrap(),
                    ));
                }

                if key.to_vec() == prefix_pair_info {
                    let pair_info: PairInfo =
                        match self.terraswap_pair_querier.pairs.get(contract_addr) {
                            Some(v) => v.clone(),
                            None => {
                                return SystemResult::Err(SystemError::InvalidRequest {
                                    error: format!("PairInfo is not found for {}", contract_addr),
                                    request: key.into(),
                                });
                            }
                        };

                    let api: MockApi = MockApi::default();
                    SystemResult::Ok(ContractResult::from(to_binary(&PairInfoRaw {
                        contract_addr: api
                            .addr_canonicalize(pair_info.contract_addr.as_str())
                            .unwrap(),
                        liquidity_token: api
                            .addr_canonicalize(pair_info.liquidity_token.as_str())
                            .unwrap(),
                        asset_infos: [
                            AssetInfoRaw::NativeToken {
                                denom: "uusd".to_string(),
                            },
                            AssetInfoRaw::NativeToken {
                                denom: "uusd".to_string(),
                            },
                        ],
                        asset_decimals: [0,0]
                    })))
                } else {
                    panic!("DO NOT ENTER HERE")
                }
            }
            _ => self.base.handle_query(request),
        }
    }
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        WasmMockQuerier {
            base,
            terraswap_pair_querier: TerraswapPairQuerier::default(),
            token_querier: TokenQuerier::default(),
            astroport_factory_querier: AstroportFactoryQuerier::default(),
        }
    }

    // configure the terraswap pair
    pub fn with_terraswap_pairs(&mut self, pairs: &[(&String, &PairInfo)]) {
        self.terraswap_pair_querier = TerraswapPairQuerier::new(pairs);
    }

    // pub fn with_balance(&mut self, balances: &[(&HumanAddr, &[Coin])]) {
    //     for (addr, balance) in balances {
    //         self.base.update_balance(addr, balance.to_vec());
    //     }
    // }

    // Configure the Astroport pair
    pub fn with_astroport_pairs(&mut self, pairs: &[(&String, &PairInfo)]) {
        self.astroport_factory_querier = AstroportFactoryQuerier::new(pairs);
    }
}
