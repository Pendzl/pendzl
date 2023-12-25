// SPDX-License-Identifier: MIT

pub use super::*;

use ink::prelude::string::String;
use pendzl::traits::Storage;
#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
    #[lazy]
    pub decimals: u8,
}

impl PSP22MetadataStorage for Data {
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

#[cfg(not(feature = "vault"))]
pub trait PSP22MetadataDefaultImpl: Storage<Data>
where
    Data: PSP22MetadataStorage,
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

#[cfg(feature = "vault")]
use crate::token::psp22::extensions::vault::{
    implementation::Data as PSP22VaultData, PSP22VaultInternal, PSP22VaultStorage,
};

#[cfg(feature = "vault")]
pub trait PSP22MetadataDefaultImpl:
    Storage<PSP22VaultData> + Storage<Data> + PSP22VaultInternal
where
    PSP22VaultData: PSP22VaultStorage,
    Data: PSP22MetadataStorage,
{
    fn token_name_default_impl(&self) -> Option<String> {
        self.data::<Data>().name.get_or_default()
    }

    fn token_symbol_default_impl(&self) -> Option<String> {
        self.data::<Data>().symbol.get_or_default()
    }

    fn token_decimals_default_impl(&self) -> u8 {
        self.data::<PSP22VaultData>().underlying_decimals() + self._decimals_offset()
    }
}
