use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, CosmosMsg, Env, Extern, HandleResponse, HumanAddr,
    InitResponse, Querier, StdError, StdResult, Storage, Uint128, WasmMsg, to_binary,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, Whitelist, STATE};

pub fn init(
    _deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    _env: Env,
    msg: InstantiateMsg,
) -> StdResult<InitResponse> {
    let config = Config {
        minter: msg.minter,
        nft_addr: msg.nft_addr,
        nft_base_uri: msg.nft_base_uri,
        nft_max_supply: msg.nft_max_supply,
        nft_price_amount: msg.nft_price_amount,
        owner: msg.owner,
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

    CONFIG.save(&mut _deps.storage, &config)?;
    STATE.save(&mut _deps.storage, &config)?;

    set_contract_version(&mut _deps.storage, "1.0")?;

    Ok(InitResponse::default())
}

pub fn execute(
    deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    msg: ExecuteMsg,
) -> StdResult<CosmosMsg> {
    match msg {
        ExecuteMsg::UpdateConfig {
            minter,
            nft_addr,
            nft_base_uri,
            nft_max_supply,
            nft_price_amount,
            owner,
        } => try_update_config(deps, env, minter, nft_addr, nft_base_uri, nft_max_supply, nft_price_amount, owner),
        ExecuteMsg::Whitelist { addrs } => try_whitelist(deps, env, addrs),
        ExecuteMsg::StartMint {} => try_start_mint(deps, env),
        ExecuteMsg::RequestMint { addr } => try_request_mint(deps, env, addr),
        ExecuteMsg::Mint {
            extension,
            token_id,
            token_uri,
        } => try_mint(deps, env, extension, token_id, token_uri),
        ExecuteMsg::Pause {} => try_pause(deps, env),
        ExecuteMsg::Unpause {} => try_unpause(deps, env),
        ExecuteMsg::WithdrawFund {} => try_withdraw_fund(deps, env),
    }
}

fn try_update_config(
    deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    minter: Option<String>,
    nft_addr: Option<Addr>,
    nft_base_uri: Option<String>,
    nft_max_supply: Option<Uint128>,
    nft_price_amount: Option<Uint128>,
    owner: Option<String>,
) -> StdResult<CosmosMsg> {
    let mut config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    if let Some(new_minter) = minter {
        config.minter = deps.api.addr_validate(&new_minter)?;
    }

    if let Some(new_nft_addr) = nft_addr {
        config.nft_addr = new_nft_addr;
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
        config.owner = deps.api.addr_validate(&new_owner)?;
    }

    CONFIG.save(&mut deps.storage, &config)?;

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::UpdateConfig {
            minter,
            nft_addr,
            nft_base_uri,
            nft_max_supply,
            nft_price_amount,
            owner,
        })?,
    }))
}

fn try_whitelist(
    deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    addrs: Vec<Addr>,
) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    let mut whitelist = Whitelist::from_storage(&mut deps.storage);
    for addr in addrs {
        whitelist.whitelist(addr)?;
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Whitelist { addrs })?,
    }))
}

fn try_start_mint(deps: &mut Extern<impl Api, impl Storage, impl Querier>, env: Env) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::StartMint {})?,
    }))
}

fn try_request_mint(
    deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    addr: Addr,
) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if !config.is_mintable {
        return Err(StdError::generic_err("Minting is not allowed at the moment."));
    }

    let whitelist = Whitelist::from_storage(&mut deps.storage);
    if !whitelist.is_whitelisted(&addr)? {
        return Err(StdError::generic_err("Address is not whitelisted for minting."));
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::RequestMint { addr })?,
    }))
}

fn try_mint(
    deps: &mut Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    extension: Option<Metadata>,
    token_id: String,
    token_uri: Option<String>,
) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Mint {
            extension,
            token_id,
            token_uri,
        })?,
    }))
}

fn try_pause(deps: &mut Extern<impl Api, impl Storage, impl Querier>, env: Env) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Pause {})?,
    }))
}

fn try_unpause(deps: &mut Extern<impl Api, impl Storage, impl Querier>, env: Env) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Unpause {})?,
    }))
}

fn try_withdraw_fund(deps: &mut Extern<impl Api, impl Storage, impl Querier>, env: Env) -> StdResult<CosmosMsg> {
    let config = CONFIG.load(&mut deps.storage)?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::WithdrawFund {})?,
    }))
}

pub fn query(
    deps: &Extern<impl Api, impl Storage, impl Querier>,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),
        QueryMsg::IsWhitelisted { addr } => to_binary(&query_is_whitelisted(deps, addr)?),
        QueryMsg::WhitelistSize {} => to_binary(&query_whitelist_size(deps)?),
        QueryMsg::TokenRequestsCount {} => to_binary(&query_token_requests_count(deps)?),
        QueryMsg::CurrentSupply {} => to_binary(&query_current_supply(deps)?),
        QueryMsg::TokenRequestByIndex { index } => {
            to_binary(&query_token_request_by_index(deps, index)?)
        }
    }
}

fn query_config(
    deps: &Extern<impl Api, impl Storage, impl Querier>,
    _env: Env,
) -> StdResult<Config> {
    Ok(CONFIG.load(&deps.storage)?)
}

fn query_is_whitelisted(
    deps: &Extern<impl Api, impl Storage, impl Querier>,
    addr: Addr,
) -> StdResult<bool> {
    let whitelist = Whitelist::from_storage(&deps.storage);
    whitelist.is_whitelisted(&addr)
}

fn query_whitelist_size(deps: &Extern<impl Api, impl Storage, impl Querier>) -> StdResult<i32> {
    let whitelist = Whitelist::from_storage(&deps.storage);
    whitelist.whitelist_size()
}

fn query_token_requests_count(deps: &Extern<impl Api, impl Storage, impl Querier>) -> StdResult<String> {
    // Replace with the actual implementation for querying token requests count
    Ok(String::from("42"))
}

fn query_current_supply(deps: &Extern<impl Api, impl Storage, impl Querier>) -> StdResult<String> {
    // Replace with the actual implementation for querying current supply
    Ok(String::from("100"))
}

fn query_token_request_by_index(
    deps: &Extern<impl Api, impl Storage, impl Querier>,
    index: Uint128,
) -> StdResult<String> {
    // Replace with the actual implementation for querying token request by index
    Ok(format!("TokenRequest at index {}", index))
}
