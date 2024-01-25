// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34)]
#[ink::contract]
pub mod my_psp34 {
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: PSP34Data,
        next_id: u8,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(&Self::env().caller(), &Id::U8(self.next_id))?;
            self.next_id = self.next_id.checked_add(1).unwrap();
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            self._mint_to(&Self::env().caller(), &id)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use ink_e2e::account_id;
        use ink_e2e::AccountKeyring::{Alice, Bob};
        use pendzl::contracts::token::psp34::Id;

        use test_helpers::{balance_of, owner_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn return_collection_id_of_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let account_id = contract.account_id;

            let expected_collection_id = Id::Bytes(AsRef::<[u8]>::as_ref(&account_id).to_vec());
            let actual_collection_id = client
                .call(&ink_e2e::alice(), &contract.collection_id())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(expected_collection_id, actual_collection_id);

            Ok(())
        }

        #[ink_e2e::test]
        async fn returns_total_supply(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let expected_total_supply = 0;
            let actual_total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?;

            assert_eq!(expected_total_supply, actual_total_supply.return_value());

            for _ in 0..3 {
                let result = client
                    .call(&ink_e2e::alice(), &contract.mint_token())
                    .submit()
                    .await
                    .expect("mint_token failed");

                assert_eq!(result.return_value(), Ok(()));
            }

            let expected_total_supply = 3;
            let actual_total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?;

            assert_eq!(expected_total_supply, actual_total_supply.return_value());

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let mint_result = client
                .call(&ink_e2e::alice(), &contract.mint_token())
                .submit()
                .await
                .expect("mint_token failed")
                .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, contract, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, contract, Bob));

            let transfer_result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .submit()
                .await
                .expect("transfer_from failed")
                .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, contract, Alice));
            assert_eq!(1, balance_of!(client, contract, Bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn approved_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let mint_result = client
                .call(&ink_e2e::alice(), &contract.mint_token())
                .submit()
                .await
                .expect("mint_token failed")
                .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, contract, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, contract, Bob));

            let approve_result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.approve(account_id(Bob), Some(Id::U8(0)), true),
                )
                .submit()
                .await
                .expect("approve failed")
                .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = client
                .call(
                    &ink_e2e::bob(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .submit()
                .await
                .expect("transfer_from failed")
                .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, contract, Alice));
            assert_eq!(1, balance_of!(client, contract, Bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn approved_operator_transfer_works(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let mint_result = client
                .call(&ink_e2e::alice(), &contract.mint_token())
                .submit()
                .await
                .expect("mint_token failed")
                .return_value();

            assert_eq!(mint_result, Ok(()));

            let expected_balance = 1;
            let actual_balance = balance_of!(client, contract, Alice);

            assert_eq!(expected_balance, actual_balance);
            assert_eq!(0, balance_of!(client, contract, Bob));

            let approve_result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.approve(account_id(Bob), None, true),
                )
                .submit()
                .await?
                .return_value();

            assert_eq!(approve_result, Ok(()));

            let transfer_result = client
                .call(
                    &ink_e2e::bob(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .submit()
                .await
                .expect("transfer_from failed")
                .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(0, balance_of!(client, contract, Alice));
            assert_eq!(1, balance_of!(client, contract, Bob));

            Ok(())
        }

        #[ink_e2e::test]
        async fn psp34_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let mint_result = client
                .call(&ink_e2e::alice(), &contract.mint_token())
                .submit()
                .await
                .expect("mint_token failed")
                .return_value();

            assert_eq!(mint_result, Ok(()));

            assert_eq!(owner_of!(client, contract, 0), Some(account_id(Alice)));

            let transfer_result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .submit()
                .await?
                .return_value();

            assert_eq!(transfer_result, Ok(()));

            assert_eq!(owner_of!(client, contract, 0), Some(account_id(Bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_not_transfer_non_existing_token(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 0);

            let transfer_result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .dry_run()
                .await?
                .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::TokenNotExists)));
            assert_eq!(balance_of!(client, contract, Alice), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_without_allowance(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let mint_result = client
                .call(&ink_e2e::alice(), &contract.mint_token())
                .submit()
                .await
                .expect("mint_token failed")
                .return_value();

            assert_eq!(mint_result, Ok(()));

            let transfer_result = client
                .call(
                    &ink_e2e::bob(),
                    &contract.transfer(account_id(Bob), Id::U8(0), vec![]),
                )
                .dry_run()
                .await?
                .return_value();

            assert!(matches!(transfer_result, Err(PSP34Error::NotApproved)));
            assert_eq!(balance_of!(client, contract, Alice), 1);
            assert_eq!(balance_of!(client, contract, Bob), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_mint_any_id(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(balance_of!(client, contract, Alice), 0);

            let ids = vec![
                Id::U8(0),
                Id::U16(0),
                Id::U32(0),
                Id::U64(0),
                Id::U128(0),
                Id::Bytes(vec![0]),
            ];

            for id in ids {
                let mint_result = client
                    .call(&ink_e2e::alice(), &contract.mint(id.clone()))
                    .submit()
                    .await?
                    .return_value();

                assert_eq!(mint_result, Ok(()));
            }

            assert_eq!(balance_of!(client, contract, Alice), 6);

            Ok(())
        }
    }
}
