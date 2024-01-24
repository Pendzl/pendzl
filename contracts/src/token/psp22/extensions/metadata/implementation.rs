// SPDX-License-Identifier: MIT

pub use super::*;

use ink::prelude::string::String;

#[allow(unused_imports)]
use pendzl::traits::Storage;
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

#[cfg(all(feature = "metadata"))]
pub trait PSP22MetadataDefaultImpl: Storage<PSP22MetadataData>
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

#[cfg(all(feature = "vault", not(feature = "metadata")))]
use crate::token::psp22::extensions::vault::{
    implementation::PSP22VaultData, PSP22VaultInternal, PSP22VaultStorage,
};

#[cfg(all(feature = "vault", not(feature = "metadata")))]
pub trait PSP22MetadataDefaultImpl:
    Storage<PSP22VaultData> + Storage<PSP22MetadataData> + PSP22VaultInternal
where
    PSP22VaultData: PSP22VaultStorage,
    PSP22Metadata: PSP22MetadataStorage,
{
    fn token_name_default_impl(&self) -> Option<String> {
        self.data::<PSP22Metadata>().name.get_or_default()
    }

    fn token_symbol_default_impl(&self) -> Option<String> {
        self.data::<PSP22Metadata>().symbol.get_or_default()
    }

    fn token_decimals_default_impl(&self) -> u8 {
        self.data::<PSP22VaultData>()
            .underlying_decimals()
            .checked_add(self._decimals_offset())
            .expect("overflow")
    }
}
