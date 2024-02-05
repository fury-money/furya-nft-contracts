use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenMetadata {
    // Add fields for token metadata based on your requirements
    pub animation_url: Option<String>,
    pub attributes: Option<Vec<TokenAttribute>>,
    pub description: Option<String>,
    pub external_url: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // Add fields for instantiation based on your requirements
    pub mint_max: Option<String>,
    pub nft_base_uri: String,
    pub nft_ci: u64,
    pub nft_max_supply: String,
    pub nft_name: String,
    pub nft_price_amount: String,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<String>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    UpdateConfig {
        minter: Option<String>,
        nft_addr: Option<String>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<String>,
        nft_price_amount: Option<String>,
        owner: Option<String>,
    },
    Whitelist {
        addrs: Vec<String>,
    },
    StartMint,
    RequestMint {
        addr: String,
    },
    Mint {
        extension: Option<TokenMetadata>,
        token_id: String,
        token_uri: Option<String>,
    },
    Pause,
    Unpause,
    WithdrawFund,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    Config,
    IsWhitelisted {
        addr: String,
    },
    WhitelistSize,
    TokenRequestsCount,
    CurrentSupply,
    TokenRequestByIndex {
        index: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub is_mintable: bool,
    pub mint_max: Option<String>,
    pub mint_start_time: u64,
    pub minter: String,
    pub nft_addr: String,
    pub nft_base_uri: String,
    pub nft_max_supply: String,
    pub nft_price_amount: String,
    pub nft_symbol: String,
    pub owner: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<String>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenRequestsCountResponse {
    pub count: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CurrentSupplyResponse {
    pub current_supply: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenRequestByIndexResponse {
    pub token_request: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WhitelistSizeResponse {
    pub whitelist_size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IsWhitelistedResponse {
    pub is_whitelisted: bool,
}
