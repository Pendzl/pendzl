// Copyright (c) 2024 C Forge. All Rights Reserved.
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
impl PSP22MetadataData {
    pub fn new(
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
    ) -> Self {
        let mut instance: PSP22MetadataData = Default::default();
        if name.is_some() {
            instance.name.set(&name);
        }
        if symbol.is_some() {
            instance.symbol.set(&symbol);
        }
        instance.decimals.set(&decimals);
        instance
    }
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
