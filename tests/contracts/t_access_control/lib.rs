// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(AccessControl)]
#[ink::contract]
pub mod t_access_control {
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        access: AccessControlData,
    }

    pub use pendzl::contracts::access_control::AccessControlInternalDefaultImpl;

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            let caller = instance.env().caller();
            // grant a caller admin role in constructor so there exist an account that can grant roles
            instance
                ._grant_role(Self::_default_admin(), Some(caller))
                .expect("caller should become admin");

            instance
        }

        #[ink(message)]
        pub fn t_grant_role(
            &mut self,
            role: RoleType,
            account: Option<AccountId>,
        ) -> Result<(), AccessControlError> {
            AccessControlInternalDefaultImpl::_grant_role_default_impl(
                self, role, account,
            )
        }

        #[ink(message)]
        pub fn t_ensure_has_role(
            &self,
            role: RoleType,
        ) -> Result<(), AccessControlError> {
            AccessControlInternalDefaultImpl::_ensure_has_role_default_impl(
                self,
                role,
                Some(Self::env().caller()),
            )
        }

        #[ink(message)]
        pub fn t_revoke_role(
            &mut self,
            role: RoleType,
            account: Option<AccountId>,
        ) -> Result<(), AccessControlError> {
            AccessControlInternalDefaultImpl::_do_revoke_role_default_impl(
                self, role, account,
            )
        }
    }
}
