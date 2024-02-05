use cosmwasm_std::{
    log, Api, Binary, CanonicalAddr, CosmosMsg, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, QueryRequest, ReadonlyStorage, StdError, StdResult, Storage,
};

use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, IsWhitelistedResponse, QueryMsg, TokenRequestByIndexResponse, TokenRequestsCountResponse, WhitelistSizeResponse};

const CONFIG_KEY: &[u8] = b"config";
const WHITELIST_KEY: &[u8] = b"whitelist";

#[derive(Default)]
pub struct Config {
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

impl Config {
    fn store<S: Storage>(&self, storage: &mut S) {
        storage.set(CONFIG_KEY, &bincode::serialize(self).unwrap());
    }

    fn load<S: Storage>(storage: &S) -> StdResult<Config> {
        storage.get(CONFIG_KEY).map(|item| match item {
            Some(data) => bincode::deserialize(&data),
            None => Ok(Default::default()),
        })
    }
}

impl Into<ConfigResponse> for Config {
    fn into(self) -> ConfigResponse {
        ConfigResponse {
            is_mintable: self.is_mintable,
            mint_max: self.mint_max,
            mint_start_time: self.mint_start_time,
            minter: self.minter,
            nft_addr: self.nft_addr,
            nft_base_uri: self.nft_base_uri,
            nft_max_supply: self.nft_max_supply,
            nft_price_amount: self.nft_price_amount,
            nft_symbol: self.nft_symbol,
            owner: self.owner,
            price_denom: self.price_denom,
            royalty_payment_address: self.royalty_payment_address,
            royalty_percentage: self.royalty_percentage,
            whitelist_mint_max: self.whitelist_mint_max,
            whitelist_mint_period: self.whitelist_mint_period,
            whitelist_mint_price_amount: self.whitelist_mint_price_amount,
        }
    }
}

pub fn init(
    _deps: &mut Extern<()>,
    _env: &Env,
    _msg: InstantiateMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse {
        log: vec![log("action", "init")],
        messages: vec![],
    })
}

pub fn handle(
    deps: &mut Extern<()>,
    env: Env,
    msg: ExecuteMsg,
) -> StdResult<HandleResponse> {
    match msg {
        ExecuteMsg::UpdateConfig { minter, nft_addr, nft_base_uri, nft_max_supply, nft_price_amount, owner } => {
            try_update_config(deps, env, minter, nft_addr, nft_base_uri, nft_max_supply, nft_price_amount, owner)
        },
        ExecuteMsg::Whitelist { addrs } => try_whitelist(deps, env, addrs),
        ExecuteMsg::StartMint => try_start_mint(deps, env),
        _ => Err(StdError::generic_err("Invalid handle message")),
    }
}

pub fn query(
    deps: &Extern<()>,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config => to_binary(&query_config(deps, env)),
        QueryMsg::IsWhitelisted { addr } => to_binary(&query_is_whitelisted(deps, addr)),
        QueryMsg::WhitelistSize => to_binary(&query_whitelist_size(deps)),
        QueryMsg::TokenRequestsCount => to_binary(&query_token_requests_count(deps)),
        QueryMsg::CurrentSupply => to_binary(&query_current_supply(deps)),
        QueryMsg::TokenRequestByIndex { index } => to_binary(&query_token_request_by_index(deps, index)),
        _ => Err(StdError::generic_err("Invalid query message")),
    }
}

fn try_update_config(
    deps: &mut Extern<()>,
    env: Env,
    minter: Option<String>,
    nft_addr: Option<HumanAddr>,
    nft_base_uri: Option<String>,
    nft_max_supply: Option<String>,
    nft_price_amount: Option<String>,
    owner: Option<String>,
) -> StdResult<HandleResponse> {
    let mut config = Config::load(&deps.storage)?;

    if let Some(new_minter) = minter {
        config.minter = new_minter;
    }
    if let Some(new_nft_addr) = nft_addr {
        config.nft_addr = new_nft_addr.to_string();
    }
    if let Some(new_nft_base_uri) = nft_base_uri {
        config.nft_base_uri = new_nft_base_uri;
    }
    if let Some(new_nft_max_supply) = nft_max_supply {
        config.nft_max_supply = new_nft_max_supply;
    }
    if let Some(new_nft_price_amount) = nft_price_amount {
        config.nft_price_amount = new_nft_price_amount;
    }
    if let Some(new_owner) = owner {
        config.owner = new_owner;
    }

    config.store(&mut deps.storage);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "update_config"),
            log("minter", &config.minter),
            log("nft_addr", &config.nft_addr),
            log("owner", &config.owner),
        ],
    })
}

fn try_whitelist(deps: &mut Extern<()>, _env: Env, addrs: Vec<String>) -> StdResult<HandleResponse> {
    let mut whitelist: Vec<CanonicalAddr> = Config::load(&deps.storage)?.whitelist;

    for addr in addrs {
        let canonical_addr = deps.api.canonical_address(&HumanAddr::from(addr))?;
        if !whitelist.contains(&canonical_addr) {
            whitelist.push(canonical_addr);
        }
    }

    let mut config = Config::load(&deps.storage)?;
    config.whitelist = whitelist;
    config.store(&mut deps.storage);

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "whitelist")],
    })
}

fn try_start_mint(deps: &mut Extern<()>, env: Env) -> StdResult<HandleResponse> {
    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("action", "start_mint")],
    })
}

fn query_config(deps: &Extern<()>, _env: Env) -> StdResult<Binary> {
    let config: Config = Config::load(&deps.storage)?;
    Ok(to_binary(&config.into())?)
}

fn query_is_whitelisted(deps: &Extern<()>, _env: Env, addr: HumanAddr) -> StdResult<Binary> {
    let config: Config = Config::load(&deps.storage)?;
    let canonical_addr = deps.api.canonical_address(&addr)?;
    let is_whitelisted = config.whitelist.contains(&canonical_addr);
    Ok(to_binary(&IsWhitelistedResponse { is_whitelisted })?)
}

fn query_whitelist_size(deps: &Extern<()>, _env: Env) -> StdResult<Binary> {
    let config: Config = Config::load(&deps.storage)?;
    let whitelist_size = config.whitelist.len() as u64;
    Ok(to_binary(&WhitelistSizeResponse { whitelist_size })?)
}

fn query_token_requests_count(deps: &Extern<()>, _env: Env) -> StdResult<Binary> {
    Ok(to_binary(&TokenRequestsCountResponse { count: 0 })?)
}

fn query_current_supply(deps: &Extern<()>, _env: Env) -> StdResult<Binary> {
    Ok(to_binary(&CurrentSupplyResponse { current_supply: "0".to_string() })?)
}

fn query_token_request_by_index(deps: &Extern<()>, _env: Env, index: Uint128) -> StdResult<Binary> {
    Ok(to_binary(&TokenRequestByIndexResponse { token_request: "".to_string() })?)
}

fn to_binary<T: serde::Serialize>(data: &T) -> StdResult<Binary> {
    Ok(Binary::from(bincode::serialize(data)?))
}

fn from_binary<T: serde::de::DeserializeOwned>(data: &[u8]) -> StdResult<T> {
    Ok(bincode::deserialize(data)?)
}
