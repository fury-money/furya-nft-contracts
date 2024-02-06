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
    pub minter: Addr,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: Addr,
    pub is_mintable: bool,
    pub mint_max: u32,
    pub mint_start_time: u64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: String,
    pub royalty_percentage: Uint128,
    pub whitelist_mint_max: u32,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Uint128,
    pub paused: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct IsWhitelistedResponse {
    pub is_whitelisted: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct WhitelistSizeResponse {
    pub whitelist_size: u32,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct TokenRequestsCountResponse {
    pub token_requests_count: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct CurrentSupplyResponse {
    pub current_supply: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct TokenRequestByIndexResponse {
    pub token_request: String,
}

pub fn init(
    deps: &mut Extern<DefaultApi, Storage, Querier>,
    env: Env,
    msg: InstantiateMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        minter: deps.api.addr_validate(&msg.minter)?,
        nft_addr: msg.nft_addr,
        nft_base_uri: msg.nft_base_uri,
        nft_max_supply: msg.nft_max_supply,
        nft_price_amount: msg.nft_price_amount,
        owner: deps.api.addr_validate(&msg.owner)?,
        is_mintable: msg.is_mintable,
        mint_max: msg.mint_max,
        mint_start_time: msg.mint_start_time,
        nft_symbol: msg.nft_symbol,
        price_denom: msg.price_denom,
        royalty_payment_address: msg.royalty_payment_address,
        royalty_percentage: msg.royalty_percentage,
        whitelist_mint_max: msg.whitelist_mint_max,
        whitelist_mint_period: msg.whitelist_mint_period,
        whitelist_mint_price_amount: msg.whitelist_mint_price_amount,
        paused: false,
    };

    config(&mut deps.storage).save(&config)?;
    state(&mut deps.storage).save(&config)?;

    set_contract_version(&mut deps.storage, "1.0")?;

    Ok(InitResponse::default())
}

// Add your tests or other helper functions here if needed
