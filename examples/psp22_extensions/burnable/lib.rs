// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// A PSP22 contract.
/// Anyone can burn tokens from anyone's account.
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// and PSP22Burnable trait's default implementation (PSP22BurnableDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP22, PSP22Burnable)]
#[ink::contract]
pub mod my_psp22_burnable {
    use pendzl::contracts::psp22::*;
    #[ink(storage)]
    // derive explained below
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22Data is a struct that implements PSP22Storage - required by PSP22InternalDefaultImpl trait
        // note it's not strictly required by PSP22 trait - just the default implementation
        // name of the field is arbitrary
        psp22: PSP22Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            // mint total_supply to the caller using _mint_to from PSP22Internal (implemented by PSP22DefaultImpl)
            instance
                ._mint_to(&Self::env().caller(), &total_supply)
                .expect("Should mint");

            instance
        }

        #[ink(message)]
        pub fn burn_from_many(
            &mut self,
            accounts: Vec<(AccountId, Balance)>,
        ) -> Result<(), PSP22Error> {
            for account in accounts.iter() {
                // use _burn_from to burn from the account from PSP22Internal (implemented by PSP22InternalDefaultImpl)
                self._burn_from(&account.0, &account.1)?;
            }
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use ink_e2e::account_id;
        use ink_e2e::AccountKeyring::{Alice, Bob, Charlie};
        use test_helpers::balance_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let balance_of_alice = balance_of!(client, contract, Alice);

            assert!(matches!(balance_of_alice, 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let result = client
                .call(&ink_e2e::alice(), &contract.burn(account_id(Alice), 10))
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_alice = balance_of!(client, contract, Alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_without_allowance(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert!(matches!(balance_of!(client, contract, Bob), 0));
            assert!(matches!(balance_of!(client, contract, Alice), 100));

            let result = client
                .call(&ink_e2e::bob(), &contract.burn(account_id(Alice), 10))
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_alice = balance_of!(client, contract, Alice);

            assert!(matches!(balance_of_alice, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn decreases_total_supply_after_burning(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?
                .return_value();

            assert!(matches!(total_supply, 100));

            let result = client
                .call(&ink_e2e::alice(), &contract.burn(account_id(Alice), 10))
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?
                .return_value();

            assert!(matches!(total_supply, 90));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 10, vec![]),
                )
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);

            assert!(matches!(balance_of_bob, 10));

            let result = client
                .call(&ink_e2e::alice(), &contract.burn(account_id(Bob), 10))
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);

            assert!(matches!(balance_of_bob, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_burn_from_many(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 10, vec![]),
                )
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Charlie), 10, vec![]),
                )
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);
            let balance_of_charlie = balance_of!(client, contract, Charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 10));

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.burn_from_many(vec![
                        (account_id(Bob), 10),
                        (account_id(Charlie), 10),
                    ]),
                )
                .submit()
                .await
                .expect("call failed")
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);
            let balance_of_charlie = balance_of!(client, contract, Charlie);

            assert!(matches!(balance_of_bob, 0));
            assert!(matches!(balance_of_charlie, 0));

            Ok(())
        }

        #[ink_e2e::test]
        async fn fails_if_one_of_the_accounts_balance_exceeds_amount_to_burn(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(100);
            let mut contract = client
                .instantiate(
                    "my_psp22_burnable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Bob), 10, vec![]),
                )
                .submit()
                .await
                .expect("call failed")
                .return_value();

            assert!(matches!(result, Ok(())));

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.transfer(account_id(Charlie), 5, vec![]),
                )
                .submit()
                .await?
                .return_value();

            assert!(matches!(result, Ok(())));

            let balance_of_bob = balance_of!(client, contract, Bob);
            let balance_of_charlie = balance_of!(client, contract, Charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            let result = client
                .call(
                    &ink_e2e::alice(),
                    &contract.burn_from_many(vec![
                        (account_id(Bob), 10),
                        (account_id(Charlie), 10),
                    ]),
                )
                .dry_run()
                .await?
                .return_value();

            assert!(matches!(result, Err(PSP22Error::InsufficientBalance)));

            let balance_of_bob = balance_of!(client, contract, Bob);
            let balance_of_charlie = balance_of!(client, contract, Charlie);

            assert!(matches!(balance_of_bob, 10));
            assert!(matches!(balance_of_charlie, 5));

            Ok(())
        }
    }
}
