use cosmwasm_std::{attr, Addr, Uint128};
use cw_multi_test::{App, Executor};

pub struct BaseContracts {
    pub anchor_token: Addr,
    pub anc_ust_pair: Addr,
    pub anc_ust: Addr,
}

/// Mint ANC tokens
pub fn mint_some_anc(
    app: &mut App,
    owner: Addr,
    anchor_token_instance: Addr,
    amount: Uint128,
    to: String,
) {
    let msg = cw20::Cw20ExecuteMsg::Mint {
        recipient: to.clone(),
        amount,
    };
    let res = app
        .execute_contract(owner.clone(), anchor_token_instance.clone(), &msg, &[])
        .unwrap();
    assert_eq!(res.events[1].attributes[1], attr("action", "mint"));
    assert_eq!(res.events[1].attributes[2], attr("to", to));
    assert_eq!(res.events[1].attributes[3], attr("amount", amount));
}

