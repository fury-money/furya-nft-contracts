use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{Addr, CosmosMsg, QuerierWrapper, StdResult, Uint128, WasmMsg};

use crate::contract::{execute, instantiate, query};
use crate::msg::{
    ConfigResponse, CurrentSupplyResponse, ExecuteMsg, InstantiateMsg, IsWhitelistedResponse,
    QueryMsg, TokenRequestByIndexResponse, TokenRequestsCountResponse, WhitelistSizeResponse,
};
use crate::helpers::FuryaBunkerMinterContract;

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        minter: String::from("minter"),
        nft_addr: Addr::unchecked("nft_address"),
        nft_base_uri: String::from("base_uri"),
        nft_max_supply: Uint128::from(1000u128),
        nft_price_amount: Uint128::from(100u128),
        owner: String::from("owner"),
        is_mintable: true,
        mint_max: Uint128::from(100u128),
        mint_start_time: 0,
        nft_symbol: String::from("NFT"),
        price_denom: String::from("uusd"),
        royalty_payment_address: String::from("royalty_address"),
        royalty_percentage: 10,
        whitelist_mint_max: Uint128::from(50u128),
        whitelist_mint_period: 1000,
        whitelist_mint_price_amount: Uint128::from(50u128),
    };
    let env = mock_env();
    let info = mock_info("creator", &[]);
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);
    let contract_addr = Addr::unchecked("contract");
    let caller = Addr::unchecked("caller");

    // Instantiate contract
    let instantiate_msg = InstantiateMsg {
        minter: String::from("minter"),
        nft_addr: Addr::unchecked("nft_address"),
        nft_base_uri: String::from("base_uri"),
        nft_max_supply: Uint128::from(1000u128),
        nft_price_amount: Uint128::from(100u128),
        owner: caller.clone().into(),
        is_mintable: true,
        mint_max: Uint128::from(100u128),
        mint_start_time: 0,
        nft_symbol: String::from("NFT"),
        price_denom: String::from("uusd"),
        royalty_payment_address: String::from("royalty_address"),
        royalty_percentage: 10,
        whitelist_mint_max: Uint128::from(50u128),
        whitelist_mint_period: 1000,
        whitelist_mint_price_amount: Uint128::from(50u128),
    };
    let instantiate_env = mock_env();
    let instantiate_info = mock_info("creator", &[]);
    let _res = instantiate(deps.as_mut(), instantiate_env, instantiate_info, instantiate_msg).unwrap();

    // Update config
    let update_config_msg = ExecuteMsg::UpdateConfig {
        minter: Some(String::from("new_minter")),
        nft_addr: Some(Addr::unchecked("new_nft_address")),
        nft_base_uri: Some(String::from("new_base_uri")),
        nft_max_supply: Some(Uint128::from(2000u128)),
        nft_price_amount: Some(Uint128::from(200u128)),
        owner: Some(String::from("new_owner")),
    };
    let update_config_env = mock_env();
    let update_config_info = mock_info(caller.as_str(), &[]);
    let res = execute(deps.as_mut(), update_config_env, update_config_info, update_config_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Query updated config
    let query_config_msg = QueryMsg::Config {};
    let query_config_response: ConfigResponse = query(deps.as_ref(), mock_env(), query_config_msg).unwrap();
    assert_eq!(query_config_response.minter, "new_minter");
    assert_eq!(query_config_response.nft_addr, Addr::unchecked("new_nft_address"));
    assert_eq!(query_config_response.nft_base_uri, "new_base_uri");
    assert_eq!(query_config_response.nft_max_supply, Uint128::from(2000u128));
    assert_eq!(query_config_response.nft_price_amount, Uint128::from(200u128));
    assert_eq!(query_config_response.owner, "new_owner");
    assert_eq!(query_config_response.is_mintable, true);
    assert_eq!(query_config_response.mint_max, Uint128::from(100u128));
    assert_eq!(query_config_response.mint_start_time, 0);
    assert_eq!(query_config_response.nft_symbol, "NFT");
    assert_eq!(query_config_response.price_denom, "uusd");
    assert_eq!(query_config_response.royalty_payment_address, "royalty_address");
    assert_eq!(query_config_response.royalty_percentage, 10);
    assert_eq!(query_config_response.whitelist_mint_max, Uint128::from(50u128));
    assert_eq!(query_config_response.whitelist_mint_period, 1000);
    assert_eq!(query_config_response.whitelist_mint_price_amount, Uint128::from(50u128));
    assert_eq!(query_config_response.paused, false);
}

// Add other integration tests here...

