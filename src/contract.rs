#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, STATE, Whitelist};

// version info for migration info
const CONTRACT_NAME: &str = "FuryaBunkerMinter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        is_mintable: msg.is_mintable,
        mint_max: msg.mint_max,
        mint_start_time: msg.mint_start_time,
        minter: msg.minter,
        nft_addr: msg.nft_addr,
        nft_base_uri: msg.nft_base_uri,
        nft_max_supply: msg.nft_max_supply,
        nft_price_amount: msg.nft_price_amount,
        nft_symbol: msg.nft_symbol,
        owner: msg.owner,
        price_denom: msg.price_denom,
        royalty_payment_address: msg.royalty_payment_address,
        royalty_percentage: msg.royalty_percentage,
        whitelist_mint_max: msg.whitelist_mint_max,
        whitelist_mint_period: msg.whitelist_mint_period,
        whitelist_mint_price_amount: msg.whitelist_mint_price_amount,
    };

    let whitelist = Whitelist { addrs: vec![] };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &whitelist)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            minter,
            nft_addr,
            nft_base_uri,
            nft_max_supply,
            nft_price_amount,
            owner,
        } => execute::update_config(
            deps,
            info,
            minter,
            nft_addr,
            nft_base_uri,
            nft_max_supply,
            nft_price_amount,
            owner,
        ),
        ExecuteMsg::Whitelist { addrs } => execute::whitelist(deps, info, addrs),
        ExecuteMsg::StartMint {} => execute::start_mint(deps, info),
        ExecuteMsg::RequestMint { addr } => execute::request_mint(deps, info, addr),
        ExecuteMsg::Mint {
            extension,
            token_id,
            token_uri,
        } => execute::mint(deps, info, extension, token_id, token_uri),
        ExecuteMsg::Pause {} => execute::pause(deps, info),
        ExecuteMsg::Unpause {} => execute::unpause(deps, info),
        ExecuteMsg::WithdrawFund {} => execute::withdraw_fund(deps, info),
    }
}

pub mod execute {
    use super::*;
    use cosmwasm_std::{Api, Querier, QuerierWrapper, QueryRequest, to_binary};

    pub fn update_config(
        deps: DepsMut,
        info: MessageInfo,
        minter: Option<String>,
        nft_addr: Option<Addr>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<Uint128>,
        nft_price_amount: Option<Uint128>,
        owner: Option<String>,
    ) -> Result<Response, ContractError> {
        // Get current config
        let mut config = CONFIG.load(deps.storage)?;

        // Ensure the sender is the owner
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }

        // Update config based on provided parameters
        config.minter = minter.unwrap_or(config.minter);
        config.nft_addr = nft_addr.unwrap_or(config.nft_addr);
        config.nft_base_uri = nft_base_uri.unwrap_or(config.nft_base_uri);
        config.nft_max_supply = nft_max_supply.unwrap_or(config.nft_max_supply);
        config.nft_price_amount = nft_price_amount.unwrap_or(config.nft_price_amount);
        config.owner = owner.unwrap_or(config.owner);

        // Save updated config
        CONFIG.save(deps.storage, &config)?;

        Ok(Response::new())
    }

    pub fn whitelist(
        deps: DepsMut,
        info: MessageInfo,
        addrs: Vec<String>,
    ) -> Result<Response, ContractError> {
        // Get current whitelist
        let mut whitelist = STATE.load(deps.storage)?;

        // Ensure the sender is the minter
        let config = CONFIG.load(deps.storage)?;
        if info.sender != config.minter {
            return Err(ContractError::Unauthorized {});
        }

        // Add addresses to the whitelist
        whitelist.addrs.extend(addrs);

        // Save updated whitelist
        STATE.save(deps.storage, &whitelist)?;

        Ok(Response::new())
    }

    pub fn start_mint(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Ensure the sender is the minter
        if info.sender != config.minter {
            return Err(ContractError::Unauthorized {});
        }

        // Your logic for starting minting process
        // ...

        Ok(Response::new())
    }

    pub fn request_mint(
        deps: DepsMut,
        info: MessageInfo,
        addr: Addr,
    ) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Your logic for handling minting request
        // ...

        Ok(Response::new())
    }

    pub fn mint(
        deps: DepsMut,
        info: MessageInfo,
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    ) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Your logic for minting a new token
        // ...

        Ok(Response::new())
    }

    pub fn pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Ensure the sender is the owner
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }

        // Your logic for pausing the contract
        // ...

        Ok(Response::new())
    }

    pub fn unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Ensure the sender is the owner
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }

        // Your logic for unpausing the contract
        // ...

        Ok(Response::new())
    }

    pub fn withdraw_fund(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Ensure the sender is the owner
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }

        // Your logic for withdrawing funds
        // ...

        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => query::config(deps),
        QueryMsg::IsWhitelisted { addr } => query::is_whitelisted(deps, addr),
        QueryMsg::WhitelistSize {} => query::whitelist_size(deps),
        QueryMsg::TokenRequestsCount {} => query::token_requests_count(deps),
        QueryMsg::CurrentSupply {} => query::current_supply(deps),
        QueryMsg::TokenRequestByIndex { index } => query::token_request_by_index(deps, index),
    }
}

pub mod query {
    use super::*;
    use cosmwasm_std::Addr;

    pub fn config(deps: Deps) -> StdResult<Binary> {
        // Get current config
        let config = CONFIG.load(deps.storage)?;

        // Your logic for returning the contract configuration
        // ...

        Ok(Binary::from(serde_json::to_string(&config).unwrap().as_bytes()))
    }

    pub fn is_whitelisted(deps: Deps, addr: Addr) -> StdResult<Binary> {
        // Get current whitelist
        let whitelist = STATE.load(deps.storage)?;

        // Your logic for checking if the address is whitelisted
        // ...

        Ok(Binary::from(serde_json::to_string(&true).unwrap().as_bytes())) // Example response, modify as needed
    }

    pub fn whitelist_size(deps: Deps) -> StdResult<Binary> {
        // Get current whitelist
        let whitelist = STATE.load(deps.storage)?;

        // Your logic for returning the size of the whitelist
        // ...

        Ok(Binary::from(serde_json::to_string(&whitelist.addrs.len()).unwrap().as_bytes()))
    }

    pub fn token_requests_count(deps: Deps) -> StdResult<Binary> {
        // Your logic for returning the count of token requests
        // ...

        Ok(Binary::from(serde_json::to_string(&0).unwrap().as_bytes())) // Example response, modify as needed
    }

    pub fn current_supply(deps: Deps) -> StdResult<Binary> {
        // Your logic for returning the current supply
        // ...

        Ok(Binary::from(serde_json::to_string(&"0".to_string()).unwrap().as_bytes())) // Example response, modify as needed
    }

    pub fn token_request_by_index(deps: Deps, index: Uint128) -> StdResult<Binary> {
        // Your logic for returning token request details by index
        // ...

        Ok(Binary::from(serde_json::to_string(&"{}").unwrap().as_bytes())) // Example response, modify as needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            is_mintable: true,
            mint_max: None,
            mint_start_time: 0,
            minter: String::from("minter"),
            nft_addr: String::from("nft_addr"),
            nft_base_uri: String::from("nft_base_uri"),
            nft_max_supply: String::from("100"),
            nft_price_amount: String::from("10"),
            nft_symbol: String::from("NFT"),
            owner: String::from("owner"),
            price_denom: String::from("uusd"),
            royalty_payment_address: None,
            royalty_percentage: None,
            whitelist_mint_max: None,
            whitelist_mint_period: 0,
            whitelist_mint_price_amount: None,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let query_res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let config: Config = from_binary(&query_res).unwrap();
        assert_eq!(config.is_mintable, true);
        // Add more assertions based on your initialization logic
    }

    // Add more tests based on your contract logic
}
