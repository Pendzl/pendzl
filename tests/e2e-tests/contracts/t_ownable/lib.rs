// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[pendzl::implementation(Ownable)]
#[ink::contract]
pub mod t_ownable {

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        ownable: OwnableData,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let mut instance = Contract::default();
            instance._update_owner(&Some(owner));
            instance
        }

        #[ink(message)]
        pub fn t_update_owner(&mut self, owner: Option<AccountId>) {
            self._update_owner(&owner)
        }

        #[ink(message)]
        pub fn t_only_owner(&mut self) -> Result<(), OwnableError> {
            self._only_owner()
        }
    }
}
