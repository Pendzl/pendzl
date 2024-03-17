// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod t_vault {
    use pendzl::contracts::psp22::{
        vault::PSP22VaultInternalDefaultImpl, PSP22InternalDefaultImpl,
    };
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
            name: String,
            symbol: String,
            max_deposit_and_mint: Option<u128>,
        ) -> Self {
            let mut instance = Self::default();
            let psp22: PSP22Ref = asset.into();
            instance.vault.asset.set(&psp22);
            let (success, asset_decimals) = instance._try_get_asset_decimals();
            let decimals_to_set = if success { asset_decimals } else { 12 };
            instance.vault.underlying_decimals.set(&decimals_to_set);

            instance.metadata.name.set(&Some(name));
            instance.metadata.symbol.set(&Some(symbol));

            instance.decimals_offset = decimals_offset;
            instance.max_deposit_and_mint = max_deposit_and_mint;

            instance
        }

        #[ink(message)]
        pub fn t_mint(
            &mut self,
            to: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_mint_to_default_impl(self, &to, &amount)
        }

        #[ink(message)]
        pub fn t_approve(
            &mut self,
            owner: AccountId,
            spender: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_approve_default_impl(
                self, &owner, &spender, &amount,
            )
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
