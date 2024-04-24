// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// A PSP22 contract with metadata.
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// and PSP22Metadata trait's default implementation (PSP22MetadataDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
#[pendzl::implementation(PSP22, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_metadata {
    use ink::prelude::string::String;

    use pendzl::contracts::psp22::PSP22Error;
    use pendzl::contracts::psp22::PSP22Internal;

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
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22Metadata>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22MetadataData is a struct that implements PSP22MetadataStorage - required by PSP22MetadataInternalDefaultImpl trait
        // note it's not strictly required by PSP22Metadata trait - just the default implementation
        // name of the field is arbitrary
        metadata: PSP22MetadataData,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            total_supply: Balance,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            // set metadata fields using lazy fields of PSP22MetadataData storage item
            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

            // mint total_supply to the caller using _update from PSP22Internal (implemented by PSP22DefaultImpl)
            instance
                ._update(None, Some(&caller), &total_supply)
                .expect("Should mint total_supply");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let _name = String::from("TOKEN");
            let _symbol = String::from("TKN");

            let mut constructor =
                ContractRef::new(1000, Some(_name), Some(_symbol), 18);
            let contract = client
                .instantiate(
                    "my_psp22_metadata",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed")
                .call_builder::<Contract>();

            let token_name = client
                .call(&ink_e2e::alice(), &contract.token_name())
                .dry_run()
                .await?
                .return_value();

            let token_symbol = client
                .call(&ink_e2e::alice(), &contract.token_symbol())
                .dry_run()
                .await?
                .return_value();

            let token_decimals = client
                .call(&ink_e2e::alice(), &contract.token_decimals())
                .dry_run()
                .await?
                .return_value();

            assert_eq!(format!("{:?}", token_name), "Some(\"TOKEN\")");
            assert_eq!(format!("{:?}", token_symbol), "Some(\"TKN\")");
            assert_eq!(format!("{:?}", token_decimals), "18");

            Ok(())
        }
    }
}
