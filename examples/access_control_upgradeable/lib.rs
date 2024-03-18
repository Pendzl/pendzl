// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, AccessControl, SetCodeHash)]
#[ink::contract]
pub mod my_access_control {

    use pendzl::contracts::psp34::{
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
        fn burn(
            &mut self,
            account: AccountId,
            id: Id,
        ) -> Result<(), PSP34Error> {
            self._ensure_has_role(MINTER, Some(self.env().caller()))?;
            self._burn_from(&account, &id)
        }
    }

    impl PSP34Mintable for Contract {
        #[ink(message)]
        fn mint(
            &mut self,
            account: AccountId,
            id: Id,
        ) -> Result<(), PSP34Error> {
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
}
