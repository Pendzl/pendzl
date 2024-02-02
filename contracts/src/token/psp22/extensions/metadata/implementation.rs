// SPDX-License-Identifier: MIT

pub use super::*;

use ink::prelude::string::String;

use pendzl::traits::StorageFieldGetter;

#[cfg(feature = "psp22_metadata_impl")]
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP22MetadataData {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
    #[lazy]
    pub decimals: u8,
}

#[cfg(feature = "psp22_metadata_impl")]
impl PSP22MetadataStorage for PSP22MetadataData {
    fn token_name(&self) -> Option<String> {
        self.name.get_or_default()
    }

    fn token_symbol(&self) -> Option<String> {
        self.symbol.get_or_default()
    }

    fn token_decimals(&self) -> u8 {
        self.decimals.get_or_default()
    }
}

#[cfg(feature = "psp22_metadata_impl")]
pub trait PSP22MetadataDefaultImpl:
    StorageFieldGetter<PSP22MetadataData>
where
    PSP22MetadataData: PSP22MetadataStorage,
{
    fn token_name_default_impl(&self) -> Option<String> {
        self.data().name.get_or_default()
    }

    fn token_symbol_default_impl(&self) -> Option<String> {
        self.data().symbol.get_or_default()
    }

    fn token_decimals_default_impl(&self) -> u8 {
        self.data().decimals.get_or_default()
    }
}

// vault metadata
#[cfg(all(
    feature = "psp22_vault_metadata_impl",
    not(feature = "psp22_metadata_impl")
))]
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP22MetadataData {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
}

#[cfg(feature = "psp22_vault_metadata_impl")]
impl PSP22VaultMetadataStorage for PSP22MetadataData {
    fn token_name(&self) -> Option<String> {
        self.name.get_or_default()
    }

    fn token_symbol(&self) -> Option<String> {
        self.symbol.get_or_default()
    }
}

#[cfg(all(
    feature = "psp22_vault_metadata_impl",
    not(feature = "psp22_metadata_impl")
))]
use crate::token::psp22::extensions::vault::{
    implementation::PSP22VaultData, PSP22VaultInternal, PSP22VaultStorage,
};
#[cfg(all(
    feature = "psp22_vault_metadata_impl",
    not(feature = "psp22_metadata_impl")
))]
pub trait PSP22MetadataDefaultImpl:
    StorageFieldGetter<PSP22VaultData>
    + StorageFieldGetter<PSP22MetadataData>
    + PSP22VaultInternal
where
    PSP22VaultData: PSP22VaultStorage,
    PSP22MetadataData: PSP22VaultMetadataStorage,
{
    fn token_name_default_impl(&self) -> Option<String> {
        self.data::<PSP22MetadataData>().name.get_or_default()
    }

    fn token_symbol_default_impl(&self) -> Option<String> {
        self.data::<PSP22MetadataData>().symbol.get_or_default()
    }

    fn token_decimals_default_impl(&self) -> u8 {
        ink::env::debug_println!(
            "underlying_decimals: {:?}",
            self.data::<PSP22VaultData>().underlying_decimals()
        );
        ink::env::debug_println!(
            "decimals_offset: {:?}",
            self._decimals_offset()
        );
        self.data::<PSP22VaultData>()
            .underlying_decimals()
            .checked_add(self._decimals_offset())
            .expect("overflow")
    }
}
