use cosmwasm_schema::{cosmwasm_serialize, from_binary, Binary};
use cosmwasm_std::{
    Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cosmwasm_std::{StdError::GenericErr, StdResult::Err};

use crate::helpers::FuryaBunkerMinterContract;

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
        nft_addr: Option<String>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<Uint128>,
        nft_price_amount: Option<Uint128>,
        owner: Option<String>,
    },
    Whitelist {
        addrs: Vec<String>,
    },
    StartMint {},
    RequestMint {
        addr: String,
    },
    Mint {
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    },
    Pause {},
    Unpause {},
    WithdrawFund {},
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
pub type Addr = String;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    pub is_mintable: bool,
    pub mint_max: Option<Uint128>,
    pub mint_start_time: i64,
    pub minter: Addr,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub nft_symbol: String,
    pub owner: Addr,
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
    StartMint {},
    RequestMint {
        addr: Addr,
    },
    Mint {
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    },
    Pause {},
    Unpause {},
    WithdrawFund {},
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
pub type ConfigResponse = Config;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct Config {
    pub is_mintable: bool,
    pub mint_max: Option<Uint128>,
    pub mint_start_time: i64,
    pub minter: Addr,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub nft_symbol: String,
    pub owner: Addr,
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub enum QueryMsg {
    Config {},
    IsWhitelisted {
        addr: Addr,
    },
    WhitelistSize {},
    TokenRequestsCount {},
    CurrentSupply {},
    TokenRequestByIndex {
        index: Uint128,
    },
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn config(&self, deps: &Deps) -> StdResult<ConfigResponse> {
        let query = QueryMsg::Config {};
        let res: ConfigResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn is_whitelisted(&self, deps: &Deps, addr: Addr) -> StdResult<IsWhitelistedResponse> {
        let query = QueryMsg::IsWhitelisted { addr };
        let res: IsWhitelistedResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn whitelist_size(&self, deps: &Deps) -> StdResult<WhitelistSizeResponse> {
        let query = QueryMsg::WhitelistSize {};
        let res: WhitelistSizeResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn token_requests_count(&self, deps: &Deps) -> StdResult<TokenRequestsCountResponse> {
        let query = QueryMsg::TokenRequestsCount {};
        let res: TokenRequestsCountResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn current_supply(&self, deps: &Deps) -> StdResult<CurrentSupplyResponse> {
        let query = QueryMsg::CurrentSupply {};
        let res: CurrentSupplyResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn token_request_by_index(
        &self,
        deps: &Deps,
        index: Uint128,
    ) -> StdResult<TokenRequestByIndexResponse> {
        let query = QueryMsg::TokenRequestByIndex { index };
        let res: TokenRequestByIndexResponse = deps.querier.query(&query.into())?;
        Ok(res)
    }

    pub fn update_config(
        &self,
        deps: &DepsMut,
        minter: Option<String>,
        nft_addr: Option<Addr>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<Uint128>,
        nft_price_amount: Option<Uint128>,
        owner: Option<String>,
    ) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::UpdateConfig {
            minter,
            nft_addr,
            nft_base_uri,
            nft_max_supply,
            nft_price_amount,
            owner,
        };
        self.call(msg)
    }

    pub fn whitelist(&self, deps: &DepsMut, addrs: Vec<Addr>) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Whitelist { addrs };
        self.call(msg)
    }

    pub fn start_mint(&self, deps: &DepsMut) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::StartMint {};
        self.call(msg)
    }

    pub fn request_mint(&self, deps: &DepsMut, addr: Addr) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::RequestMint { addr };
        self.call(msg)
    }

    pub fn mint(
        &self,
        deps: &DepsMut,
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    ) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Mint {
            extension,
            token_id,
            token_uri,
        };
        self.call(msg)
    }

    pub fn pause(&self, deps: &DepsMut) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Pause {};
        self.call(msg)
    }

    pub fn unpause(&self, deps: &DepsMut) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Unpause {};
        self.call(msg)
    }

    pub fn withdraw_fund(&self, deps: &DepsMut) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::WithdrawFund {};
        self.call(msg)
    }
}

pub mod execute {
    use super::*;

    pub fn update_config(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        minter: Option<String>,
        nft_addr: Option<Addr>,
        nft_base_uri: Option<String>,
        nft_max_supply: Option<Uint128>,
        nft_price_amount: Option<Uint128>,
        owner: Option<String>,
    ) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn whitelist(deps: DepsMut, env: Env, info: MessageInfo, addrs: Vec<Addr>) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn start_mint(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn request_mint(deps: DepsMut, env: Env, info: MessageInfo, addr: Addr) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn mint(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        extension: Option<Metadata>,
        token_id: String,
        token_uri: Option<String>,
    ) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn pause(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn unpause(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
        unimplemented!();
    }

    pub fn withdraw_fund(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
        unimplemented!();
    }
}

pub mod query {
    use super::*;

    pub fn config(deps: Deps, env: Env) -> Result<Binary, StdError> {
        unimplemented!();
    }

    pub fn is_whitelisted(deps: Deps, env: Env, addr: Addr) -> Result<Binary, StdError> {
        unimplemented!();
    }

    pub fn whitelist_size(deps: Deps, env: Env) -> Result<Binary, StdError> {
        unimplemented!();
    }

    pub fn token_requests_count(deps: Deps, env: Env) -> Result<Binary, StdError> {
        unimplemented!();
    }

    pub fn current_supply(deps: Deps, env: Env) -> Result<Binary, StdError> {
        unimplemented!();
    }

    pub fn token_request_by_index(deps: Deps, env: Env, index: Uint128) -> Result<Binary, StdError> {
        unimplemented!();
    }
}
