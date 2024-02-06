use cosmwasm_std::{
    Addr, Api, Binary, BlockInfo, ChannelResponse, CosmosMsg, Env, Response, StdError, StdResult,
    Storage, Uint128, WasmMsg, to_binary,
};
use cw2::set_contract_version;

use crate::msg::Metadata;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, config, Whitelist, whitelist};

pub fn init(
    _deps: &mut Extern<DefaultApi, Storage, Querier>,
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

    config(&mut _deps.storage).save(&config)?;
    state(&mut _deps.storage).save(&config)?;

    set_contract_version(&mut _deps.storage, "1.0")?;

    Ok(InitResponse::default())
}

pub fn execute(
    deps: &mut Extern<DefaultApi, Storage, Querier>,
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
    deps: &mut Extern<DefaultApi, Storage, Querier>,
    env: Env,
    minter: Option<String>,
    nft_addr: Option<Addr>,
    nft_base_uri: Option<String>,
    nft_max_supply: Option<Uint128>,
    nft_price_amount: Option<Uint128>,
    owner: Option<String>,
) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
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

    config(&mut deps.storage).save(&config)?;

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
    deps: &mut Extern<DefaultApi, Storage, Querier>,
    env: Env,
    addrs: Vec<Addr>,
) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    let mut whitelist = whitelist(&mut deps.storage);
    for addr in addrs {
        whitelist.whitelist(addr)?;
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Whitelist { addrs })?,
    }))
}

fn try_start_mint(deps: &mut Extern<DefaultApi, Storage, Querier>, env: Env) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
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
    deps: &mut Extern<DefaultApi, Storage, Querier>,
    env: Env,
    addr: Addr,
) -> StdResult<CosmosMsg> {
    let config = config(&mut deps.storage).load()?;
    if !config.is_mintable {
        return Err(StdError::generic_err("Minting is not allowed at the moment."));
    }

    let whitelist = whitelist(&mut deps.storage);
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
    deps: &mut Extern<DefaultApi, Storage, Querier>,
    env: Env,
    extension: Option<Metadata>,
    token_id: String,
    token_uri: Option<String>,
) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
    if !config.is_mintable {
        return Err(StdError::generic_err("Minting is not allowed at the moment."));
    }

    // Perform minting logic here...

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

fn try_pause(deps: &mut Extern<DefaultApi, Storage, Querier>, env: Env) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    config.paused = true;
    config(&mut deps.storage).save(&config)?;

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Pause {})?,
    }))
}

fn try_unpause(deps: &mut Extern<DefaultApi, Storage, Querier>, env: Env) -> StdResult<CosmosMsg> {
    let mut config = config(&mut deps.storage).load()?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    config.paused = false;
    config(&mut deps.storage).save(&config)?;

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Unpause {})?,
    }))
}

fn try_withdraw_fund(deps: &mut Extern<DefaultApi, Storage, Querier>, env: Env) -> StdResult<CosmosMsg> {
    let config = config(&mut deps.storage).load()?;
    if config.owner != deps.api.canonical_address(&env.message.sender)? {
        return Err(StdError::unauthorized());
    }

    // Perform fund withdrawal logic here...

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address,
        funds: vec![],
        msg: to_binary(&ExecuteMsg::WithdrawFund {})?,
    }))
}

pub fn query(
    deps: &Extern<DefaultApi, Storage, Querier>,
    msg: QueryMsg,
) -> StdResult<Response> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::IsWhitelisted { addr } => to_binary(&query_is_whitelisted(deps, addr)?),
        QueryMsg::WhitelistSize {} => to_binary(&query_whitelist_size(deps)?),
        QueryMsg::TokenRequestsCount {} => to_binary(&query_token_requests_count(deps)?),
        QueryMsg::CurrentSupply {} => to_binary(&query_current_supply(deps)?),
        QueryMsg::TokenRequestByIndex { index } => {
            to_binary(&query_token_request_by_index(deps, index)?)
        }
    }
}

fn query_config(deps: &Extern<DefaultApi, Storage, Querier>) -> StdResult<ConfigResponse> {
    let config = config(&deps.storage).load()?;
    Ok(ConfigResponse {
        minter: config.minter,
        nft_addr: config.nft_addr,
        nft_base_uri: config.nft_base_uri,
        nft_max_supply: config.nft_max_supply,
        nft_price_amount: config.nft_price_amount,
        owner: config.owner,
        is_mintable: config.is_mintable,
        mint_max: config.mint_max,
        mint_start_time: config.mint_start_time,
        nft_symbol: config.nft_symbol,
        price_denom: config.price_denom,
        royalty_payment_address: config.royalty_payment_address,
        royalty_percentage: config.royalty_percentage,
        whitelist_mint_max: config.whitelist_mint_max,
        whitelist_mint_period: config.whitelist_mint_period,
        whitelist_mint_price_amount: config.whitelist_mint_price_amount,
        paused: config.paused,
    })
}

fn query_is_whitelisted(
    deps: &Extern<DefaultApi, Storage, Querier>,
    addr: Addr,
) -> StdResult<IsWhitelistedResponse> {
    let whitelist = whitelist(&deps.storage);
    let is_whitelisted = whitelist.is_whitelisted(&addr)?;
    Ok(IsWhitelistedResponse { is_whitelisted })
}

fn query_whitelist_size(deps: &Extern<DefaultApi, Storage, Querier>) -> StdResult<WhitelistSizeResponse> {
    let whitelist = whitelist(&deps.storage);
    let whitelist_size = whitelist.whitelist_size()?;
    Ok(WhitelistSizeResponse { whitelist_size })
}

fn query_token_requests_count(deps: &Extern<DefaultApi, Storage, Querier>) -> StdResult<TokenRequestsCountResponse> {
    // Perform token requests count logic here...
    Ok(TokenRequestsCountResponse { token_requests_count: "TODO".to_string() })
}

fn query_current_supply(deps: &Extern<DefaultApi, Storage, Querier>) -> StdResult<CurrentSupplyResponse> {
    // Perform current supply logic here...
    Ok(CurrentSupplyResponse { current_supply: "TODO".to_string() })
}

fn query_token_request_by_index(
    deps: &Extern<DefaultApi, Storage, Querier>,
    index: Uint128,
) -> StdResult<TokenRequestByIndexResponse> {
    // Perform token request by index logic here...
    Ok(TokenRequestByIndexResponse { token_request: "TODO".to_string() })
}
