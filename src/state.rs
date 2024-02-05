use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    Querier, Deps, DepsMut, QueryResponse, to_binary, Binary, StdResult, QuerierWrapper,
    CustomQuery, WasmQuery,
};

use crate::error::ContractError;
use crate::msg::{QueryMsg, ConfigResponse, IsWhitelistedResponse, WhitelistSizeResponse, TokenRequestsCountResponse, CurrentSupplyResponse, TokenRequestByIndexResponse};


use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub minter: Addr,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: Addr,
    pub is_mintable: bool,
    pub mint_max: Option<Uint128>,
    pub mint_start_time: i64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<Uint128>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<Uint128>,
    pub paused: bool,
}

pub const STATE: Item<State> = Item::new("state");

// Add any additional methods or functions related to state handling if needed
impl State {
    // Example method to get the current configuration
    pub fn get_config<Q, CQ>(&self, querier: &Q) -> StdResult<ConfigResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::Config {})?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // Example method to check if an address is whitelisted
    pub fn is_whitelisted<Q, CQ>(&self, querier: &Q, addr: Addr) -> StdResult<IsWhitelistedResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::IsWhitelisted { addr })?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // ... Add more state-related methods based on your contract logic

    // Example method to get the current whitelist size
    pub fn get_whitelist_size<Q, CQ>(&self, querier: &Q) -> StdResult<WhitelistSizeResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::WhitelistSize {})?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // ... Add more state-related methods based on your contract logic

    // Example method to get the current token requests count
    pub fn get_token_requests_count<Q, CQ>(&self, querier: &Q) -> StdResult<TokenRequestsCountResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::TokenRequestsCount {})?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // ... Add more state-related methods based on your contract logic

    // Example method to get the current supply
    pub fn get_current_supply<Q, CQ>(&self, querier: &Q) -> StdResult<CurrentSupplyResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::CurrentSupply {})?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // ... Add more state-related methods based on your contract logic

    // Example method to get token request by index
    pub fn get_token_request_by_index<Q, CQ>(
        &self,
        querier: &Q,
        index: Uint128,
    ) -> StdResult<TokenRequestByIndexResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let query = WasmQuery::Smart {
            contract_addr: env.contract.address.into(),
            msg: to_binary(&QueryMsg::TokenRequestByIndex { index })?,
        }
        .into();
        QuerierWrapper::<CQ>::new(querier).query(&query)
    }

    // ... Add more state-related methods based on your contract logic
}