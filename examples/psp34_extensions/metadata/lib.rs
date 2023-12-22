// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP34, PSP34Metadata)]
#[ink::contract]
pub mod my_psp34_metadata {
    use ink::prelude::string::*;
    use pendzl::contracts::token::psp34::*;
    #[derive(Default, Storage)]
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
        pub fn new(id: Id, name: String, symbol: String) -> Self {
            let mut instance = Self::default();

            let name_key = String::from("name");
            let symbol_key = String::from("symbol");
            instance._set_attribute(&id.clone(), &name_key, &name);
            instance._set_attribute(&id, &symbol_key, &symbol);

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;
        use pendzl::contracts::token::psp34::Id;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let id = Id::U8(0);
            let name = String::from("My PSP34");
            let symbol = String::from("MPS34");

            let mut constructor = ContractRef::new(id.clone(), name.clone(), symbol.clone());
            let contract = client
                .instantiate("my_psp34_metadata", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<Contract>();

            let result_name = client
                .call(
                    &ink_e2e::alice(),
                    &contract.get_attribute(id.clone(), String::from("name")),
                )
                .dry_run()
                .await?
                .return_value();

            let result_symbol = client
                .call(
                    &ink_e2e::alice(),
                    &contract.get_attribute(id.clone(), String::from("symbol")),
                )
                .dry_run()
                .await?
                .return_value();

            assert_eq!(result_name, Some(name));
            assert_eq!(result_symbol, Some(symbol));

            Ok(())
        }
    }
}
