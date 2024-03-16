// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[pendzl::implementation(PSP22, PSP22Metadata)]
#[ink::contract]
pub mod t_psp22 {
    use ink::prelude::string::String;
    use pendzl::contracts::psp22::PSP22Error;
    use pendzl::contracts::psp22::PSP22Internal;
    use pendzl::contracts::psp22::PSP22InternalDefaultImpl;

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
        #[ink(message)]
        pub fn t_mint(
            &mut self,
            to: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_mint_to_default_impl(self, &to, &amount)
        }
        #[ink(message)]
        pub fn t_burn(
            &mut self,
            from: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_burn_from_default_impl(
                self, &from, &amount,
            )
        }

        #[ink(message)]
        pub fn t_update(
            &mut self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_update_default_impl(
                self,
                from.as_ref(),
                to.as_ref(),
                &amount,
            )
        }
        #[ink(message)]
        pub fn t_transfer(
            &mut self,
            from: AccountId,
            to: AccountId,
            amount: Balance,
        ) -> Result<(), PSP22Error> {
            PSP22InternalDefaultImpl::_transfer_default_impl(
                self, &from, &to, &amount,
            )
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
}
