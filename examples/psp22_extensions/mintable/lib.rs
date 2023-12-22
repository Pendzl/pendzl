// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Mintable)]
#[ink::contract]
pub mod my_psp22_mintable {
    use pendzl::contracts::token::psp22::*;
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

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
        async fn assigns_initial_balance(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new(1000);
            let contract = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert!(matches!(balance_of!(client, contract, Alice), 1000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn minting_requested_amount(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new(1000);
            let mut contract = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

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
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

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

            assert!(matches!(total_supply, 1000), "Total supply should be 1000");

            Ok(())
        }
    }
}
