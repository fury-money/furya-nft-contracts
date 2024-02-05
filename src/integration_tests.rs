#[cfg(test)]
mod tests {
    use crate::helpers::FuryaBunkerMinter;
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, FuryaBunkerMinter) {
        let mut app = mock_app();
        let furya_bunker_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            is_mintable: true,
            mint_max: None,
            mint_start_time: 0,
            minter: Addr::unchecked(ADMIN),
            nft_addr: Addr::unchecked(USER),
            nft_base_uri: "https://example.com".to_string(),
            nft_max_supply: Uint128::new(100),
            nft_price_amount: Uint128::new(1),
            nft_symbol: "NFT".to_string(),
            owner: Addr::unchecked(ADMIN),
            price_denom: NATIVE_DENOM.to_string(),
            royalty_payment_address: None,
            royalty_percentage: None,
            whitelist_mint_max: None,
            whitelist_mint_period: 0,
            whitelist_mint_price_amount: None,
        };
        let furya_bunker_contract_addr = app
            .instantiate_contract(
                furya_bunker_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let furya_bunker_contract = FuryaBunkerMinter(furya_bunker_contract_addr);

        (app, furya_bunker_contract)
    }

    mod mint {
        use super::*;
        use crate::msg::Metadata;

        #[test]
        fn mint() {
            let (mut app, furya_bunker_contract) = proper_instantiate();

            let metadata = Metadata {
                animation_url: Some("https://example.com/animation".to_string()),
                attributes: None,
                description: Some("Description".to_string()),
                external_url: Some("https://example.com/external".to_string()),
                image: Some("https://example.com/image".to_string()),
                name: Some("Token Name".to_string()),
                royalty_payment_address: None,
                royalty_percentage: None,
            };

            let msg = ExecuteMsg::Mint {
                extension: Some(metadata),
                token_id: "1".to_string(),
                token_uri: Some("https://example.com/token/1".to_string()),
            };

            let cosmos_msg = furya_bunker_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
