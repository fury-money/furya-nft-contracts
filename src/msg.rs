use cosmwasm_schema::{to_binary, from_binary, Binary};
use cosmwasm_std::{
    Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct InstantiateMsg {
    pub mint_max: Option<Uint128>,
    pub nft_base_uri: String,
    pub nft_ci: u64,
    pub nft_max_supply: Uint128,
    pub nft_name: String,
    pub nft_price_amount: Uint128,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<Uint128>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<Uint128>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub enum ExecuteMsg {
    UpdateConfig {
        minter: Option<String>,
        nft_addr: Option<Addr>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<Uint128>,
        nft_price_amount: Option<Uint128>,
        owner: Option<String>,
    },
    Whitelist {
        addrs: Vec<Addr>,
    },
    StartMint,
    RequestMint {
        addr: Addr,
    },
    Mint {
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    },
    Pause,
    Unpause,
    WithdrawFund,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Metadata {
    pub animation_url: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub description: Option<String>,
    pub external_url: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct ConfigResponse {
    pub is_mintable: bool,
    pub mint_max: Option<Uint128>,
    pub mint_start_time: i64,
    pub minter: String,
    pub nft_addr: String,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub nft_symbol: String,
    pub owner: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<Uint128>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<Uint128>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub type CurrentSupplyResponse = String;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub type IsWhitelistedResponse = bool;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub type TokenRequestByIndexResponse = String;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub type TokenRequestsCountResponse = String;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub type WhitelistSizeResponse = i32;

// Add your tests or other helper functions here if needed
