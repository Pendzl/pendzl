use pendzl::traits::{StorageFieldGetter, String};

use crate::token::psp22::extensions::vault::{
    implementation::PSP22VaultData, PSP22VaultInternal, PSP22VaultStorage,
};

use super::PSP22VaultMetadataStorage;

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct PSP22MetadataData {
    #[lazy]
    pub name: Option<String>,
    #[lazy]
    pub symbol: Option<String>,
}

impl PSP22VaultMetadataStorage for PSP22MetadataData {
    fn token_name(&self) -> Option<String> {
        self.name.get_or_default()
    }

    fn token_symbol(&self) -> Option<String> {
        self.symbol.get_or_default()
    }
}

pub trait PSP22MetadataDefaultImpl:
    StorageFieldGetter<PSP22VaultData> + StorageFieldGetter<PSP22MetadataData> + PSP22VaultInternal
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
        self.data::<PSP22VaultData>()
            .underlying_decimals()
            .checked_add(self._decimals_offset())
            .expect("overflow")
    }
}
