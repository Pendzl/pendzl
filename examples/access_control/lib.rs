// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, AccessControl)]
#[ink::contract]
pub mod my_access_control {

    use pendzl::contracts::token::psp34::extensions::{
        burnable::PSP34Burnable, mintable::PSP34Mintable,
    };

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp34: PSP34Data,
        #[storage_field]
        access: AccessControlData,
    }

    // You can manually set the number for the role.
    // But better to use a hash of the variable name.
    // It will generate a unique identifier of this role.
    // And will reduce the chance to have overlapping roles.
    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl PSP34Burnable for Contract {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._ensure_has_role(MINTER, Some(self.env().caller()))?;
            self._burn_from(&account, &id)
        }
    }

    impl PSP34Mintable for Contract {
        #[ink(message)]
        fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            self._ensure_has_role(MINTER, Some(self.env().caller()))?;
            self._mint_to(&account, &id)
        }
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            // grant a caller admin role in constructor so there exist an account that can grant roles
            instance
                ._grant_role(Self::_default_admin(), Some(caller))
                .expect("caller should become admin");
            // grant minter role to caller in constructor, so he can mint/burn tokens
            instance
                ._grant_role(MINTER, Some(caller))
                .expect("Should grant MINTER role");

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
        use ink_e2e::AccountKeyring::{Alice, Bob, Charlie};
        use ink_e2e::{alice, bob};
        use pendzl::contracts::access::access_control::DEFAULT_ADMIN_ROLE;
        use test_helpers::{grant_role, has_role, revoke_role};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn only_minter_role_is_allowed_to_mint(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);

            // bob can not mint
            assert_eq!(
                format!(
                    "{:?}",
                    client
                        .call(&bob(), &mut contract.mint(account_id(Bob), Id::U8(1)))
                        .dry_run()
                        .await?
                        .return_value()
                ),
                "Err(Custom(\"AC::MissingRole\"))"
            );

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, contract, MINTER, Bob), true);

            assert_eq!(
                client
                    .call(&bob(), &mut contract.mint(account_id(Bob), Id::U8(0)))
                    .submit()
                    .await?
                    .return_value(),
                Ok(())
            );

            let owner_of = client
                .call(&alice(), &contract.owner_of(Id::U8(0)))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner_of, Some(account_id(Bob)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_initial_roles_to_default_signer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Alice), true);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_grant_initial_roles_for_random_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_grant_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, contract, MINTER, Bob), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_not_change_old_roles_after_grant_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, contract, MINTER, Alice), true);

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, contract, MINTER, Bob), true);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Bob), false);
            assert_eq!(has_role!(client, contract, DEFAULT_ADMIN_ROLE, Alice), true);
            assert_eq!(has_role!(client, contract, MINTER, Alice), true);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_revoke_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));

            assert_eq!(has_role!(client, contract, MINTER, Bob), true);

            let revoke_role_res = client
                .call(
                    &alice(),
                    &contract.revoke_role(MINTER, Some(account_id(Bob))),
                )
                .submit()
                .await?
                .return_value();
            assert_eq!(format!("{:?}", revoke_role_res), "Ok(())");

            assert_eq!(has_role!(client, contract, MINTER, Bob), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_renounce_role(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(has_role!(client, contract, MINTER, Alice), true);

            let renounce_role_res = client
                .call(
                    &alice(),
                    &contract.renounce_role(MINTER, Some(account_id(Alice))),
                )
                .submit()
                .await?
                .return_value();
            assert_eq!(format!("{:?}", renounce_role_res), "Ok(())");

            assert_eq!(has_role!(client, contract, MINTER, Alice), false);

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_grant_or_revoke_not_by_admin_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));

            let grant_role_res = client
                .call(
                    &ink_e2e::bob(),
                    &contract.grant_role(MINTER, Some(account_id(Charlie))),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", grant_role_res), "Err(MissingRole)");

            let revoke_role_res = client
                .call(
                    &ink_e2e::bob(),
                    &contract.revoke_role(MINTER, Some(account_id(Charlie))),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", revoke_role_res), "Err(MissingRole)");

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_when_renounce_not_self_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, contract, MINTER, Bob), true);

            let renounce_role_res = client
                .call(
                    &alice(),
                    &contract.renounce_role(MINTER, Some(account_id(Bob))),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", renounce_role_res), "Err(InvalidCaller)");

            Ok(())
        }

        #[ink_e2e::test]
        async fn should_reject_burn_if_no_minter_role(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let mut constructor = ContractRef::new();
            let mut contract = client
                .instantiate("my_access_control", &alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            assert_eq!(grant_role!(client, contract, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, contract, MINTER, Bob), true);

            assert_eq!(
                client
                    .call(&bob(), &mut contract.mint(account_id(Bob), Id::U8(0)))
                    .submit()
                    .await?
                    .return_value(),
                Ok(())
            );

            let owner_of = client
                .call(&ink_e2e::bob(), &contract.owner_of(Id::U8(0)))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(owner_of, Some(account_id(Bob)));

            assert_eq!(revoke_role!(client, contract, MINTER, Bob), Ok(()));
            assert_eq!(has_role!(client, contract, MINTER, Bob), false);

            let burn_res = client
                .call(&ink_e2e::bob(), &contract.burn(account_id(Bob), Id::U8(0)))
                .dry_run()
                .await?
                .return_value();

            assert_eq!(
                format!("{:?}", burn_res),
                "Err(Custom(\"AC::MissingRole\"))"
            );

            Ok(())
        }
    }
}
