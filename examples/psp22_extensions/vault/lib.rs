// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

/// A PSP22 vault contract with metadata decimal_offset and optional max_deposit_and_mint.
// inject PSP22 trait's default implementation (PSP22DefaultImpl & PSP22InternalDefaultImpl)
// and PSP22Vault trait's default implementation (PSP22VaultDefaultImpl & PSP22VaultInternalDefaultImpl)
// and PSP22Metadata trait's default implementation (PSP22MetadataDefaultImpl)
// which reduces the amount of boilerplate code required to implement trait messages drastically
// Note: since PSP22Vault is used, the PSP22Metadata implementation that is used is PSP22VaultMetadata, not the default one
#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_vault {
    use ink::prelude::string::ToString;
    use pendzl::traits::String;
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
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22Vault>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22VaultData is a struct that implements PSP22VaultStorage - required by PSP22VaultInternalDefaultImpl trait
        // note it's not strictly required by PSP22Vault trait - just the default implementation
        // name of the field is arbitrary
        vault: PSP22VaultData,
        // apply the storage_field attribute so it's accessible via `self.data::<PSP22Metadata>()` (provided by StorageFieldGetter derive)
        #[storage_field]
        // PSP22MetadataData is a struct that implements PSP22MetadataStorage - required by PSP22MetadataInternalDefaultImpl trait
        // note it's not strictly required by PSP22Metadata trait - just the default implementation
        // name of the field is arbitrary
        metadata: PSP22MetadataData,

        //additional fields
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
            Self {
                psp22: PSP22Data::default(),
                vault: PSP22VaultData::new(asset, None),
                metadata: PSP22MetadataData::new(
                    Some("Name".to_string()),
                    Some("Symbol".to_string()),
                ),
                decimals_offset,
                max_deposit_and_mint,
            }
        }
    }

    // override _decimals_offset from PSP22VaultInternal trait's default implementation (PSP22VaultInternalDefaultImpl)
    #[overrider(PSP22VaultInternal)]
    fn _decimals_offset(&self) -> u8 {
        self.decimals_offset
    }

    // override _max_deposit from PSP22VaultInternal trait's default implementation (PSP22VaultInternalDefaultImpl)
    #[overrider(PSP22VaultInternal)]
    fn _max_deposit(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            // call the default implementation
            PSP22VaultInternalDefaultImpl::_max_deposit_default_impl(self, to)
        }
    }
    // override _max_mint from PSP22VaultInternal trait's default implementation (PSP22VaultInternalDefaultImpl)
    #[overrider(PSP22VaultInternal)]
    fn _max_mint(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            // call the default implementation
            PSP22VaultInternalDefaultImpl::_max_mint_default_impl(self, to)
        }
    }
}
