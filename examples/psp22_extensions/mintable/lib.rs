// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// A PSP22 contract with mintable extension - anyone can mint tokens.
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// and PSP22Mintable trait's default implementation (PSP22MintableDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP22, PSP22Mintable)]
#[ink::contract]
pub mod my_psp22_mintable {
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
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use ink_e2e::account_id;
        use ink_e2e::AccountKeyring::{Alice, Bob};
        use test_helpers::balance_of;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(1000);
            let contract = client
                .instantiate(
                    "my_psp22_mintable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert!(matches!(balance_of!(client, contract, Alice), 1000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn minting_requested_amount(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(1000);
            let mut contract = client
                .instantiate(
                    "my_psp22_mintable",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            assert!(
                matches!(balance_of!(client, contract, Bob), 0),
                "Bob's balance should be 0"
            );

            let mint_tx = client
                .call(&ink_e2e::alice(), &contract.mint(account_id(Bob), 1000))
                .submit()
                .await
                .expect("transfer failed")
                .return_value();

            assert!(matches!(mint_tx, Ok(())), "Minting should be successful");

            assert!(
                matches!(balance_of!(client, contract, Bob), 1000),
                "Bob's balance should be 1000"
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn increases_total_supply_after_minting(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new(0);
            let mut contract = client
                .instantiate(
                    "my_psp22_mintable",
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

            assert!(matches!(total_supply, 0), "Total supply should be 0");

            let mint_tx = client
                .call(&ink_e2e::alice(), &contract.mint(account_id(Bob), 1000))
                .submit()
                .await
                .expect("transfer failed")
                .return_value();

            assert!(matches!(mint_tx, Ok(())), "Minting should be successful");

            let total_supply = client
                .call(&ink_e2e::alice(), &contract.total_supply())
                .dry_run()
                .await?
                .return_value();

            assert!(
                matches!(total_supply, 1000),
                "Total supply should be 1000"
            );

            Ok(())
        }
    }
}
