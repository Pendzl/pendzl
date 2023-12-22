// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, PSP34Mintable)]
#[ink::contract]
pub mod my_psp34_mintable {
    use pendzl::contracts::token::psp34::*;
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: PSP34Data,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::AccountKeyring::{Alice, Bob};

        use ink_e2e::account_id;
        use ink_e2e::ContractsBackend;
        use test_helpers::balance_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 0);
            assert_eq!(balance_of!(client, contract, Bob), 0);

            let id_1 = Id::U8(1);
            let id_2 = Id::U8(2);

            let mint_1 = client
                .call(
                    &ink_e2e::alice(),
                    &contract.mint(account_id(Alice), id_1.clone()),
                )
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = client
                .call(
                    &ink_e2e::alice(),
                    &contract.mint(account_id(Bob), id_2.clone()),
                )
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert_eq!(mint_2, Ok(()));

            assert_eq!(balance_of!(client, contract, Alice), 1);
            assert_eq!(balance_of!(client, contract, Bob), 1);

            Ok(())
        }

        #[ink_e2e::test]
        async fn mint_existing_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 0);
            assert_eq!(balance_of!(client, contract, Bob), 0);

            let id_1 = Id::U8(1);

            let mint_1 = client
                .call(
                    &ink_e2e::alice(),
                    &contract.mint(account_id(Alice), id_1.clone()),
                )
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = client
                .call(
                    &ink_e2e::alice(),
                    &contract.mint(account_id(Bob), id_1.clone()),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", mint_2), "Err(TokenExists)");

            assert_eq!(balance_of!(client, contract, Alice), 1);
            assert_eq!(balance_of!(client, contract, Bob), 0);

            Ok(())
        }
    }
}
