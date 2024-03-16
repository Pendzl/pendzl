// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, PSP34Metadata)]
#[ink::contract]
pub mod t_psp34_metadata {
    use pendzl::contracts::psp34::*;
    use pendzl::traits::String;

    const COLLECTION_ID: Id = Id::U8(0);

    #[derive(Default, StorageFieldGetter)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp34: PSP34Data,
        #[storage_field]
        metadata: PSP34MetadataData,
    }

    impl Contract {
        /// A constructor which mints the first token to the owner
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            let id = COLLECTION_ID;
            instance._set_attribute(&id.clone(), &name_key, &name);
            instance._set_attribute(&id, &symbol_key, &symbol);

            instance
        }

        #[ink(message)]
        pub fn t_mint(
            &mut self,
            to: AccountId,
            id: Id,
        ) -> Result<(), PSP34Error> {
            self._mint_to(&to, &id)
        }

        #[ink(message)]
        pub fn t_burn(
            &mut self,
            from: AccountId,
            id: Id,
        ) -> Result<(), PSP34Error> {
            self._burn_from(&from, &id)
        }

        #[ink(message)]
        pub fn t_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: Id,
            data: Vec<u8>,
        ) -> Result<(), PSP34Error> {
            self._transfer(&from, &to, &id, &data)
        }

        #[ink(message)]
        pub fn t_update(
            &mut self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            id: Id,
        ) -> Result<(), PSP34Error> {
            self._update(&from.as_ref(), &to.as_ref(), &id)
        }

        #[ink(message)]
        pub fn t_set_atribute(
            &mut self,
            id: Id,
            key: String,
            atribute: String,
        ) {
            self._set_attribute(&id, &key, &atribute)
        }
    }
}
