use cosmwasm_std::{
    Addr, CosmosMsg, Deps, DepsMut, Env, Extern, MessageInfo, Response, StdError, StdResult, Storage,
    Querier, InitResponse,
};
use cosmwasm_schema::{to_binary, from_binary, Binary};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use cw_storage_plus::{Item, Map};

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
pub struct Config {
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

pub const CONFIG_KEY: &[u8] = b"config";
pub const STATE_KEY: &[u8] = b"state";
pub const WHITELIST_KEY: &[u8] = b"whitelist";

pub fn config(storage: &mut dyn Storage) -> Item<Config> {
    Item::new(storage, CONFIG_KEY)
}

pub fn state(storage: &mut dyn Storage) -> Item<Config> {
    Item::new(storage, STATE_KEY)
}

pub fn whitelist(storage: &mut dyn Storage) -> Map<Whitelist> {
    Map::new(storage, WHITELIST_KEY)
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

    config(deps.storage()).save(&config)?;

    Ok(InitResponse::default())
}
