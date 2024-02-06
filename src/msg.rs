use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub minter: String,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: String,
    pub is_mintable: bool,
    pub mint_max: Uint128,
    pub mint_start_time: u64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: String,
    pub royalty_percentage: u32,
    pub whitelist_mint_max: Uint128,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ExecuteMsg {
    #[serde(rename = "update_config")]
    pub update_config: Option<UpdateConfigMsg>,
    pub whitelist: Option<WhitelistMsg>,
    #[serde(rename = "start_mint")]
    pub start_mint: Option<StartMintMsg>,
    #[serde(rename = "request_mint")]
    pub request_mint: Option<RequestMintMsg>,
    pub mint: Option<MintMsg>,
    pub pause: Option<PauseMsg>,
    pub unpause: Option<UnpauseMsg>,
    #[serde(rename = "withdraw_fund")]
    pub withdraw_fund: Option<WithdrawFundMsg>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct UpdateConfigMsg {
    pub minter: Option<String>,
    pub nft_addr: Option<Addr>,
    pub nft_base_uri: Option<String>,
    pub nft_max_supply: Option<Uint128>,
    pub nft_price_amount: Option<Uint128>,
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct WhitelistMsg {
    pub addrs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct StartMintMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct RequestMintMsg {
    pub addr: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct MintMsg {
    pub extension: Option<Metadata>,
    pub token_id: String,
    pub token_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Metadata {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct PauseMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct UnpauseMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct WithdrawFundMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    IsWhitelisted { addr: String },
    WhitelistSize {},
    TokenRequestsCount {},
    CurrentSupply {},
    TokenRequestByIndex { index: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub minter: String,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: String,
    pub is_mintable: bool,
    pub mint_max: Uint128,
    pub mint_start_time: u64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: String,
    pub royalty_percentage: u32,
    pub whitelist_mint_max: Uint128,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Uint128,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct IsWhitelistedResponse {
    pub is_whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct WhitelistSizeResponse {
    pub whitelist_size: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TokenRequestsCountResponse {
    pub token_requests_count: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct CurrentSupplyResponse {
    pub current_supply: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct TokenRequestByIndexResponse {
    pub token_request: String,
}
