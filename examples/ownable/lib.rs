// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// An PSP22 contract with ownable module.
/// A creator of the contract becomes an owner.
/// Owner is allowed to mint and burn PSP22 tokens.
#[pendzl::implementation(PSP22, Ownable)]
#[ink::contract]
pub mod ownable {
    use pendzl::contracts::psp22::{
        burnable::PSP22Burnable, mintable::PSP22Mintable,
    };

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        ownable: OwnableData,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Contract::default();
            instance._update_owner(&Some(Self::env().caller()));
            instance
        }
    }

    impl PSP22Burnable for Contract {
        #[ink(message)]
        fn burn(
            &mut self,
            account: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            self._only_owner()?;
            self._update(Some(&account), None, &amount)
        }
    }

    impl PSP22Mintable for Contract {
        #[ink(message)]
        fn mint(
            &mut self,
            account: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            self._only_owner()?;
            self._update(None, Some(&account), &amount)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        use ink_e2e::{
            account_id, alice,
            AccountKeyring::{Alice, Bob},
            ContractsBackend,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn owner_is_by_default_contract_deployer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let mint_res = client
                .call(&alice(), &contract.mint(account_id(Bob), 1))
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert!(matches!(mint_res, Ok(())));

            Ok(())
        }
    }
}
