use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};

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
