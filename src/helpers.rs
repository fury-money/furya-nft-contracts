use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::Metadata;

use cosmwasm_std::{
    Addr, Binary, CosmosMsg, Querier, QuerierWrapper, StdResult, Uint128, WasmMsg, WasmQuery,
    CustomQuery, to_binary,
};

use crate::msg::{ExecuteMsg, QueryMsg};
use crate::msg::{
    ConfigResponse, CurrentSupplyResponse, IsWhitelistedResponse, TokenRequestByIndexResponse,
    TokenRequestsCountResponse, WhitelistSizeResponse,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct FuryaBunkerMinterContract(pub Addr);

impl FuryaBunkerMinterContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn config<Q, CQ>(&self, querier: &Q) -> StdResult<ConfigResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::Config {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: ConfigResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn is_whitelisted<Q, CQ>(&self, querier: &Q, addr: Addr) -> StdResult<IsWhitelistedResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::IsWhitelisted { addr: addr.to_string() };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: IsWhitelistedResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn whitelist_size<Q, CQ>(&self, querier: &Q) -> StdResult<WhitelistSizeResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::WhitelistSize {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: WhitelistSizeResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn token_requests_count<Q, CQ>(&self, querier: &Q) -> StdResult<TokenRequestsCountResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::TokenRequestsCount {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: TokenRequestsCountResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn current_supply<Q, CQ>(&self, querier: &Q) -> StdResult<CurrentSupplyResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::CurrentSupply {};
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: CurrentSupplyResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn token_request_by_index<Q, CQ>(
        &self,
        querier: &Q,
        index: Uint128,
    ) -> StdResult<TokenRequestByIndexResponse>
    where
        Q: Querier,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::TokenRequestByIndex { index };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&msg)?,
        }
        .into();
        let res: TokenRequestByIndexResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }

    pub fn update_config<T>(&self, minter: Option<T>) -> StdResult<CosmosMsg>
    where
        T: Into<String>,
    {
        let msg = ExecuteMsg::UpdateConfig { minter: minter.map(Into::into) };
        self.call(msg)
    }

    pub fn whitelist<T>(&self, addrs: Vec<T>) -> StdResult<CosmosMsg>
    where
        T: Into<String>,
    {
        let msg = ExecuteMsg::Whitelist { addrs: addrs.into_iter().map(Into::into).collect() };
        self.call(msg)
    }

    pub fn start_mint(&self) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::StartMint {};
        self.call(msg)
    }

    pub fn request_mint<T>(&self, addr: T) -> StdResult<CosmosMsg>
    where
        T: Into<String>,
    {
        let msg = ExecuteMsg::RequestMint { addr: addr.into() };
        self.call(msg)
    }

    pub fn mint<T, U, V>(
        &self,
        extension: Option<Metadata>,
        token_id: T,
        token_uri: Option<U>,
    ) -> StdResult<CosmosMsg>
    where
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        let msg = ExecuteMsg::Mint {
            extension,
            token_id: token_id.into(),
            token_uri: token_uri.map(Into::into),
        };
        self.call(msg)
    }

    pub fn pause(&self) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Pause {};
        self.call(msg)
    }

    pub fn unpause(&self) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::Unpause {};
        self.call(msg)
    }

    pub fn withdraw_fund(&self) -> StdResult<CosmosMsg> {
        let msg = ExecuteMsg::WithdrawFund {};
        self.call(msg)
    }
}
