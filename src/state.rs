use cosmwasm_std::{Addr, StdResult, Storage};
use serde::{Serialize, Deserialize};
use cosmwasm_std::Uint128;
use cosmwasm_std::storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Config {
    pub minter: String,
    pub nft_addr: Addr,
    pub nft_base_uri: String,
    pub nft_max_supply: Uint128,
    pub nft_price_amount: Uint128,
    pub owner: String,
    pub is_mintable: bool,
    pub mint_max: Uint128,
    pub mint_start_time: u64,
    pub nft_symbol: String,
    pub price_denom: String,
    pub royalty_payment_address: String,
    pub royalty_percentage: u32,
    pub whitelist_mint_max: Uint128,
    pub whitelist_mint_period: u64,
    pub whitelist_mint_price_amount: Uint128,
    pub paused: bool,
}

pub fn config<S: Storage + ?Sized>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, b"config")
}

pub fn read_config<S: Storage + ?Sized>(storage: &S) -> ReadonlySingleton<S, Config> {
    singleton_read(storage, b"config")
}

pub fn state<S: Storage + ?Sized>(storage: &mut S) -> Singleton<S, Config> {
    singleton(storage, b"state")
}

pub fn read_state<S: Storage + ?Sized>(storage: &S) -> ReadonlySingleton<S, Config> {
    singleton_read(storage, b"state")
}

impl Config {
    pub fn save<S: Storage + ?Sized>(&self, storage: &mut S) -> StdResult<()> {
        config(storage).save(self)
    }

    pub fn load<S: Storage + ?Sized>(storage: &S) -> StdResult<Config> {
        read_config(storage).load()
    }

    pub fn state_save<S: Storage + ?Sized>(&self, storage: &mut S) -> StdResult<()> {
        state(storage).save(self)
    }

    pub fn state_load<S: Storage + ?Sized>(storage: &S) -> StdResult<Config> {
        read_state(storage).load()
    }
}
