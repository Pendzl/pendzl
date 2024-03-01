// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_vault {
    use ink::prelude::string::ToString;
    use pendzl::contracts::token::psp22::extensions::vault::implementation::PSP22VaultInternalDefaultImpl;
    use pendzl::traits::String;
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        vault: PSP22VaultData,
        #[storage_field]
        metadata: PSP22MetadataData,

        decimals_offset: u8,

        max_deposit_and_mint: Option<u128>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            asset: AccountId,
            decimals_offset: u8,
            max_deposit_and_mint: Option<u128>,
        ) -> Self {
            let mut instance = Self {
                psp22: PSP22Data::default(),
                vault: PSP22VaultData::new(asset, None),
                metadata: PSP22MetadataData::new(
                    Some("Name".to_string()),
                    Some("Symbol".to_string()),
                ),
                decimals_offset,
                max_deposit_and_mint,
            };

            instance
        }
    }

    #[overrider(PSP22VaultInternal)]
    fn _decimals_offset(&self) -> u8 {
        self.decimals_offset
    }

    #[overrider(PSP22VaultInternal)]
    fn _max_deposit(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            PSP22VaultInternalDefaultImpl::_max_deposit_default_impl(self, to)
        }
    }
    #[overrider(PSP22VaultInternal)]
    fn _max_mint(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            PSP22VaultInternalDefaultImpl::_max_mint_default_impl(self, to)
        }
    }
}
