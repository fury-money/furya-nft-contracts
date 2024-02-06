use cosmwasm_std::{Addr, StdResult, Storage, ReadonlyStorage};
use serde::{Serialize, Deserialize};
use cosmwasm_std::Uint128;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

const CONFIG_KEY: &[u8] = b"config";
const STATE_KEY: &[u8] = b"state";
const WHITELIST_KEY: &[u8] = b"whitelist";

pub fn config<S: Storage>(storage: &S) -> Singleton<S, Config> {
    singleton(storage, CONFIG_KEY)
}

pub fn read_config<S: ReadonlyStorage>(storage: &S) -> ReadonlySingleton<S, Config> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn state<S: Storage>(storage: &S) -> Singleton<S, Config> {
    singleton(storage, STATE_KEY)
}

pub fn read_state<S: ReadonlyStorage>(storage: &S) -> ReadonlySingleton<S, Config> {
    singleton_read(storage, STATE_KEY)
}

pub fn whitelist<S: Storage>(storage: &S) -> Singleton<S, Whitelist> {
    singleton(storage, WHITELIST_KEY)
}

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

impl Config {
    pub fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        config(storage).save(self)
    }

    pub fn load<S: ReadonlyStorage>(storage: &S) -> StdResult<Config> {
        read_config(storage).load()
    }

    pub fn state_save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {
        state(storage).save(self)
    }

    pub fn state_load<S: ReadonlyStorage>(storage: &S) -> StdResult<Config> {
        read_state(storage).load()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Whitelist {
    // Define your Whitelist struct fields here
    pub addresses: Vec<Addr>,
}

impl Whitelist {
    pub fn new() -> Self {
        Whitelist {
            addresses: Vec::new(),
        }
    }

    pub fn add_address(&mut self, address: Addr) {
        self.addresses.push(address);
    }

    pub fn is_whitelisted(&self, address: &Addr) -> bool {
        self.addresses.contains(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn test_whitelist() {
        let mut storage = MockStorage::new();
        let mut whitelist = whitelist(&mut storage);

        let addr1 = Addr::unchecked("addr1");
        let addr2 = Addr::unchecked("addr2");

        assert!(!whitelist.is_whitelisted(&addr1));

        whitelist.add_address(addr1.clone());
        assert!(whitelist.is_whitelisted(&addr1));
        assert!(!whitelist.is_whitelisted(&addr2));
    }
}
