// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[pendzl::implementation(PSP22)]
#[ink::contract]
pub mod my_psp22 {
    use ink::prelude::string::String;
    #[ink::storage_item]
    #[derive(Debug)]
    pub struct HatedStorage {
        pub hated_account: AccountId,
    }

    #[ink(storage)]
    #[derive(StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        hated_storage: HatedStorage,
    }

    #[overrider(PSP22Internal)]
    fn _update(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if to == Some(&self.hated_storage.hated_account) {
            return Err(PSP22Error::Custom(String::from(
                "I hate this account!",
            )));
        }
        pendzl::contracts::psp22::PSP22InternalDefaultImpl::_update_default_impl(
            self, from, to, amount,
        )
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                psp22: Default::default(),
                hated_storage: HatedStorage {
                    hated_account: [255; 32].into(),
                },
            };

            instance
                ._mint_to(&Self::env().caller(), &total_supply)
                .expect("Should mint");

            instance
        }

        #[ink(message)]
        pub fn set_hated_account(&mut self, account: AccountId) {
            self.hated_storage.hated_account = account;
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use super::*;
        use ink_e2e::account_id;
        use ink_e2e::AccountKeyring::{Alice, Bob};
        use ink_e2e::ContractsBackend;
        use test_helpers::balance_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let balance_of_deployer = client
                .call(
                    &ink_e2e::alice(),
                    &contract.balance_of(account_id(Alice)),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(balance_of_deployer, 100);

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let result = {
                client
                    .call(
                        &ink_e2e::alice(),
                        &contract.transfer(account_id(Bob), 50, vec![]),
                    )
                    .submit()
                    .await
                    .expect("transfer failed")
                    .return_value()
            };

            assert!(matches!(result, Ok(())));

            let balance_of_alice = balance_of!(client, contract, Alice);

            let balance_of_bob = balance_of!(client, contract, Bob);

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 101, vec![]),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", result), "Err(InsufficientBalance)");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_to_hated_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate("my_psp22", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 10, vec![]),
                )
                .submit()
                .await
                .expect("transfer failed")
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);

            assert_eq!(balance_of_bob, 10);

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.set_hated_account(account_id(Bob)),
                )
                .submit()
                .await
                .expect("set_hated_account failed")
                .return_value();

            assert!(matches!(result, ()));

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 10, vec![]),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                format!("{:?}", result),
                "Err(Custom(\"I hate this account!\"))"
            );

            let balance_of_bob = balance_of!(client, contract, Bob);

            assert!(matches!(balance_of_bob, 10));

            Ok(())
        }
    }
}
