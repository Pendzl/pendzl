// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_metadata {
    use ink::prelude::string::String;

    use pendzl::contracts::psp22::PSP22Error;
    use pendzl::contracts::psp22::PSP22Internal;

    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
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

            instance.metadata.name.set(&name);
            instance.metadata.symbol.set(&symbol);
            instance.metadata.decimals.set(&decimal);

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
