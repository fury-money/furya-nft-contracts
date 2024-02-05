use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub minter: Addr,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: Addr,
    pub is_mintable: bool,
    pub mint_max: Option<Uint128>,
    pub mint_start_time: i64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: Option<String>,
    pub royalty_percentage: Option<u32>,
    pub whitelist_mint_max: Option<Uint128>,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Option<Uint128>,
    pub paused: bool,
}

pub const STATE: Item<State> = Item::new("state");
