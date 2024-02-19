// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_vault {
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
            let mut instance = Self::default();
            let psp22: PSP22Ref = asset.into();
            instance.vault.asset.set(&psp22);
            let (success, asset_decimals) = instance._try_get_asset_decimals();
            let decimals_to_set = if success { asset_decimals } else { 12 };
            instance.vault.underlying_decimals.set(&decimals_to_set);

            instance.decimals_offset = decimals_offset;
            instance.max_deposit_and_mint = max_deposit_and_mint;

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
