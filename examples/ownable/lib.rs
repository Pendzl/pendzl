// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::storage::Mapping;
use pendzl::traits::AccountId;
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Ownable2Data {
    #[lazy]
    pub lazy_owner: Option<AccountId>,
    pub owner: Option<AccountId>,

    pub map: Mapping<AccountId, u128>,
}

#[pendzl::implementation(PSP22, Ownable)]
#[ink::contract]
pub mod ownable {
    use pendzl::contracts::token::psp22::extensions::{
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
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._only_owner()?;
            self._update(Some(&account), None, &amount)
        }
    }

    impl PSP22Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._only_owner()?;
            self._update(None, Some(&account), &amount)
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
        use ink_e2e::{alice, bob};

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
                .call::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_owner_is_allowed_to_mint(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

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

        #[ink_e2e::test]
        async fn transfer_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let mint_res = client
                .call(&bob(), &contract.mint(account_id(Bob), 123))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                format!("{:?}", mint_res),
                "Err(Custom(\"O::CallerIsNotOwner\"))"
            );

            let balance_before = client
                .call(&alice(), &contract.balance_of(account_id(Bob)))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(balance_before, 0);

            let transfer_ownership_tx = client
                .call(&alice(), &contract.transfer_ownership(account_id(Bob)))
                .submit()
                .await?
                .return_value();

            assert_eq!(transfer_ownership_tx, Ok(()));

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Bob)));

            let mint_res = client
                .call(&bob(), &contract.mint(account_id(Bob), 123))
                .submit()
                .await
                .expect("mint failed")
                .return_value();

            assert_eq!(mint_res, Ok(()));

            let balance_after = client
                .call(&alice(), &contract.balance_of(account_id(Bob)))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(balance_after, 123);

            Ok(())
        }

        #[ink_e2e::test]
        async fn renounce_ownership_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let renounce_ownership_tx = client
                .call(&alice(), &contract.renounce_ownership())
                .submit()
                .await
                .expect("renounce_ownership failed")
                .return_value();

            assert_eq!(renounce_ownership_tx, Ok(()));

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, None);

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_renounce_ownership_if_not_owner(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let renounce_ownership_tx = client
                .call(&bob(), &contract.renounce_ownership())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                format!("{:?}", renounce_ownership_tx),
                "Err(CallerIsNotOwner)"
            );

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_ownership_if_not_owner(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_ownable", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            let renounce_ownership_tx = client
                .call(&bob(), &contract.renounce_ownership())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                format!("{:?}", renounce_ownership_tx),
                "Err(CallerIsNotOwner)"
            );

            let owner = client
                .call(&alice(), &contract.owner())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner, Some(account_id(Alice)));

            Ok(())
        }
    }
}
