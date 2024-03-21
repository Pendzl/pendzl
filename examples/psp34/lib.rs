// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34)]
#[ink::contract]
pub mod my_psp34 {
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
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
        use ink_e2e::AccountKeyring::Alice;
        use pendzl::contracts::psp34::Id;

        use test_helpers::balance_of;

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
                .call_builder::<Contract>();

            let account_id = contract.account_id;

            let expected_collection_id =
                Id::Bytes(AsRef::<[u8]>::as_ref(&account_id).to_vec());
            let actual_collection_id = client
                .call(&ink_e2e::alice(), &contract.collection_id())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(expected_collection_id, actual_collection_id);

            Ok(())
        }

        #[ink_e2e::test]
        async fn returns_total_supply(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let expected_total_supply = 0;
            let actual_total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?;

            assert_eq!(
                expected_total_supply,
                actual_total_supply.return_value()
            );

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

            assert_eq!(
                expected_total_supply,
                actual_total_supply.return_value()
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_mint_any_id(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_psp34", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

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
