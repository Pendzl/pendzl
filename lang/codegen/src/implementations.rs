// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT
use quote::ToTokens;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::{punctuated::Punctuated, token::Comma, Block, FnArg};

pub type OverridenFnMap = HashMap<
    String,
    Vec<(
        String,
        (Box<Block>, Vec<syn::Attribute>, Punctuated<FnArg, Comma>),
    )>,
>;

pub struct ImplArgs<'a> {
    pub map: &'a OverridenFnMap,
    pub items: &'a mut Vec<syn::Item>,
    pub imports: &'a mut HashMap<&'a str, syn::ItemUse>,
    pub overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
    pub storage_struct_name: String,
}

impl<'a> ImplArgs<'a> {
    pub fn new(
        map: &'a OverridenFnMap,
        items: &'a mut Vec<syn::Item>,
        imports: &'a mut HashMap<&'a str, syn::ItemUse>,
        overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
        storage_struct_name: String,
    ) -> Self {
        Self {
            map,
            items,
            imports,
            overriden_traits,
            storage_struct_name,
        }
    }

    fn contract_name(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.storage_struct_name)
    }

    fn vec_import(&mut self) {
        let vec_import = syn::parse2::<syn::ItemUse>(quote!(
            use ink::prelude::vec::Vec;
        ))
        .expect("Should parse");
        self.imports.insert("vec", vec_import);
    }
}

pub(crate) fn impl_psp22(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::PSP22InternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::psp22::PSP22Internal for #storage_struct_name {
            fn _total_supply(&self) -> Balance {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_total_supply_default_impl(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_balance_of_default_impl(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_allowance_default_impl(self, owner, spender)
            }

            fn _update(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_update_default_impl(self, from, to, amount)
            }

            fn _transfer(
                &mut self,
                from: &AccountId,
                to: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22Internal::_update(self, Some(from), Some(to), amount)
            }

            fn _mint_to(
                &mut self,
                to: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22Internal::_update(self, None, Some(to), amount)
            }

            fn _burn_from(
                &mut self,
                from: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22Internal::_update(self, Some(from), None, amount)
            }

            fn _approve(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_approve_default_impl(self, owner, spender, amount)
            }

            fn _decrease_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_decrease_allowance_from_to_default_impl(self, owner, spender, amount)
            }

            fn _increase_allowance_from_to(
                &mut self,
                owner: &AccountId,
                spender: &AccountId,
                amount: &Balance,
            ) -> Result<(), PSP22Error>{
                pendzl::contracts::psp22::PSP22InternalDefaultImpl::_increase_allowance_from_to_default_impl(self, owner, spender, amount)

            }
        }
    ))
    .expect("Should parse");

    let psp22_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::PSP22DefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::PSP22 for #storage_struct_name {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                pendzl::contracts::psp22::PSP22DefaultImpl::total_supply_default_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                pendzl::contracts::psp22::PSP22DefaultImpl::balance_of_default_impl(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                pendzl::contracts::psp22::PSP22DefaultImpl::allowance_default_impl(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22DefaultImpl::transfer_default_impl(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22DefaultImpl::transfer_from_default_impl(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22DefaultImpl::approve_default_impl(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22DefaultImpl::increase_allowance_default_impl(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::PSP22DefaultImpl::decrease_allowance_default_impl(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::PSP22Data;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP22", import);
    impl_args.imports.insert("PSP22Data", import_data);
    impl_args.vec_import();

    override_functions("PSP22Internal", &mut internal, impl_args.map);
    override_functions("PSP22", &mut psp22, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp22_default_impl));
    impl_args.items.push(syn::Item::Impl(psp22));
}

pub(crate) fn impl_psp22_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::burnable::PSP22BurnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::burnable::PSP22Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::burnable::PSP22BurnableDefaultImpl::burn_default_impl(self,from,amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::burnable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Burnable", import);
    impl_args.vec_import();

    override_functions("PSP22Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_default_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp22_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::mintable::PSP22MintableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::mintable::PSP22Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, to: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::mintable::PSP22MintableDefaultImpl::mint_default_impl(self, to, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::mintable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Mintable", import);
    impl_args.vec_import();

    override_functions("PSP22Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_default_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp22_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let metadata_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::metadata::PSP22MetadataDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::metadata::PSP22Metadata for #storage_struct_name {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                pendzl::contracts::psp22::metadata::PSP22MetadataDefaultImpl::token_name_default_impl(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                pendzl::contracts::psp22::metadata::PSP22MetadataDefaultImpl::token_symbol_default_impl(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                pendzl::contracts::psp22::metadata::PSP22MetadataDefaultImpl::token_decimals_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::metadata::*;
    ))
    .expect("Should parse");
    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::metadata::PSP22MetadataData;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Metadata", import);
    impl_args.imports.insert("PSP22MetadataData", import_data);
    impl_args.vec_import();

    override_functions("PSP22Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(metadata_default_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp22_vault(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::vault::PSP22VaultInternal for #storage_struct_name {
            fn _decimals_offset(&self) -> u8 {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_decimals_offset_default_impl(self)
            }

            fn _try_get_asset_decimals(&self) -> (bool, u8) {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_try_get_asset_decimals_default_impl(self)
            }

            fn _asset(&self) -> PSP22Ref {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_asset_default_impl(self)
            }

            fn _total_assets(&self) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_total_assets_default_impl(self)
            }

            fn _convert_to_shares(&self, assets: &Balance, rounding: Rounding) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_convert_to_shares_default_impl(self, assets, rounding)
            }

            fn _convert_to_assets(&self, shares: &Balance, rounding: Rounding) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_convert_to_assets_default_impl(self, shares, rounding)

            }

            fn _max_deposit(&self, to: &AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_max_deposit_default_impl(self, to)
            }

            fn _max_mint(&self, to: &AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_max_mint_default_impl(self, to)
            }

            fn _max_withdraw(&self, owner: &AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_max_withdraw_default_impl(self, owner)
            }

            fn _max_redeem(&self, owner: &AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_max_redeem_default_impl(self, owner)
            }

            fn _preview_deposit(&self, assets: &Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_preview_deposit_default_impl(self, assets)
            }

            fn _preview_mint(&self, shares: &Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_preview_mint_default_impl(self, shares)
            }

            fn _preview_withdraw(&self, assets: &Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_preview_withdraw_default_impl(self, assets)
            }

            fn _preview_redeem(&self, shares: &Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_preview_redeem_default_impl(self, shares)
            }

            fn _deposit(
                &mut self,
                caller: &AccountId,
                receiver: &AccountId,
                assets: &Balance,
                shares: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_deposit_default_impl(self, caller, receiver, assets, shares)
            }

            fn _withdraw(
                &mut self,
                caller: &AccountId,
                receiver: &AccountId,
                owner: &AccountId,
                assets: &Balance,
                shares: &Balance,
            ) -> Result<(), PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultInternalDefaultImpl::_withdraw_default_impl(self, caller, receiver, owner, assets, shares)
            }
        }
    ))
    .expect("Should parse");

    let psp22_vault_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp22_vault = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp22::vault::PSP22Vault for #storage_struct_name {
            #[ink(message)]
            fn asset(&self) -> AccountId {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::asset_default_impl(self)
            }

            #[ink(message)]
            fn total_assets(&self) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::total_assets_default_impl(self)
            }

            #[ink(message)]
            fn convert_to_shares(&self, assets: Balance, round: Rounding) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::convert_to_shares_default_impl(self, assets, round)
            }

            #[ink(message)]
            fn convert_to_assets(&self, shares: Balance, round: Rounding) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::convert_to_assets_default_impl(self, shares, round)
            }

            #[ink(message)]
            fn max_deposit(&self, to: AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::max_deposit_default_impl(self, to)
            }

            #[ink(message)]
            fn max_mint(&self, to: AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::max_mint_default_impl(self, to)
            }

            #[ink(message)]
            fn max_withdraw(&self, owner: AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::max_withdraw_default_impl(self, owner)
            }

            #[ink(message)]
            fn max_redeem(&self, owner: AccountId) -> Balance {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::max_redeem_default_impl(self, owner)
            }

            #[ink(message)]
            fn preview_deposit(&self, assets: Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::preview_deposit_default_impl(self, assets)
            }

            #[ink(message)]
            fn preview_mint(&self, shares: Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::preview_mint_default_impl(self, shares)
            }

            #[ink(message)]
            fn preview_withdraw(&self, assets: Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::preview_withdraw_default_impl(self, assets)
            }

            #[ink(message)]
            fn preview_redeem(&self, shares: Balance) -> Result<Balance, MathError> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::preview_redeem_default_impl(self, shares)
            }

            #[ink(message)]
            fn deposit(&mut self, assets: Balance, receiver: AccountId) -> Result<Balance, PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::deposit_default_impl(self, assets, receiver)
            }

            #[ink(message)]
            fn mint(&mut self, shares: Balance, receiver: AccountId) -> Result<Balance, PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::mint_default_impl(self, shares, receiver)
            }

            #[ink(message)]
            fn withdraw(&mut self, assets: Balance, receiver: AccountId, owner: AccountId) -> Result<Balance, PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::withdraw_default_impl(self, assets, receiver, owner)
            }

            #[ink(message)]
            fn redeem(&mut self, shares: Balance, receiver: AccountId, owner: AccountId) -> Result<Balance, PSP22Error> {
                pendzl::contracts::psp22::vault::PSP22VaultDefaultImpl::redeem_default_impl(self, shares, receiver, owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::vault::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::vault::PSP22VaultData;
    ))
    .expect("Should parse import");

    let import_rounding = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp22::vault::Rounding;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP22Vault", import);
    impl_args.imports.insert("PSP22VaultData", import_data);
    impl_args
        .imports
        .insert("PSP22VaultRounding", import_rounding);
    impl_args.vec_import();

    override_functions("PSP22VaultInternal", &mut internal, impl_args.map);
    override_functions("PSP22Vault", &mut psp22_vault, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args
        .items
        .push(syn::Item::Impl(psp22_vault_default_impl));
    impl_args.items.push(syn::Item::Impl(psp22_vault));
}

pub(crate) fn impl_psp34(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::PSP34InternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::PSP34Internal for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId) -> u32 {
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_balance_of_default_impl(self, owner)
            }

            fn _total_supply(&self) -> u64 {
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_total_supply_default_impl(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_owner_of_default_impl(self,id)
            }

            fn _allowance(&self, owner: &AccountId, operator: &AccountId, id: &Option<Id>) -> bool {
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_allowance_default_impl(self, owner, operator, id)
            }

            fn _approve(&mut self, owner: &AccountId,operator: &AccountId, id: &Option<Id>, approved: &bool) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_approve_default_impl(self, owner, operator, id, approved)
            }

            fn _update(
                &mut self,
                from: &Option<&AccountId>,
                to: &Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error>{
                pendzl::contracts::psp34::PSP34InternalDefaultImpl::_update_default_impl(self, from, to, id)

            }

            fn _transfer(&mut self, from: &AccountId, to: &AccountId, id: &Id, data: &Vec<u8>) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34Internal::_update(self, &Some(from), &Some(to), id)
            }

            fn _mint_to(&mut self, to: &AccountId, id: &Id) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34Internal::_update(self, &None, &Some(to), id)
            }

            fn _burn_from(&mut self, from: &AccountId, id: &Id) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34Internal::_update(self, &Some(from), &None, id)
            }

        }
    ))
    .expect("Should parse");

    let psp34_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::PSP34DefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34 = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::PSP34 for #storage_struct_name {
            #[ink(message)]
            fn collection_id(&self) -> Id {
                pendzl::contracts::psp34::PSP34DefaultImpl::collection_id_default_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> u32 {
                pendzl::contracts::psp34::PSP34DefaultImpl::balance_of_default_impl(self, owner)
            }

            #[ink(message)]
            fn owner_of(&self, id: Id) -> Option<AccountId> {
                pendzl::contracts::psp34::PSP34DefaultImpl::owner_of_default_impl(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
                pendzl::contracts::psp34::PSP34DefaultImpl::allowance_default_impl(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34DefaultImpl::approve_default_impl(self, operator, id, approved)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::PSP34DefaultImpl::transfer_default_impl(self, to, id, data)
            }

            #[ink(message)]
            fn total_supply(&self) -> u64 {
                pendzl::contracts::psp34::PSP34DefaultImpl::total_supply_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::PSP34Data;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP34", import);
    impl_args.imports.insert("PSP34Data", import_data);
    impl_args.vec_import();

    override_functions("PSP34Internal", &mut internal, impl_args.map);
    override_functions("PSP34", &mut psp34, impl_args.map);

    // only insert this if it is not present
    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp34_default_impl));
    impl_args.items.push(syn::Item::Impl(psp34));
}

pub(crate) fn impl_psp34_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::burnable::PSP34BurnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::burnable::PSP34Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::burnable::PSP34BurnableDefaultImpl::burn_default_impl(self,from,id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::burnable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP34Burnable", import);
    impl_args.vec_import();

    override_functions("PSP34Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_default_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp34_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::mintable::PSP34MintableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::mintable::PSP34Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                pendzl::contracts::psp34::mintable::PSP34MintableDefaultImpl::mint_default_impl(self,from,id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::mintable::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP34Mintable", import);
    impl_args.vec_import();

    override_functions("PSP34Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_default_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp34_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::metadata::PSP34MetadataInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::metadata::PSP34MetadataInternal for #storage_struct_name {

            fn _set_attribute(&mut self, id: &Id, key: &String, value: &String) {
                pendzl::contracts::psp34::metadata::PSP34MetadataInternalDefaultImpl::_set_attribute_default_impl(self, id, key, value)
            }
        }
    ))
    .expect("Should parse");

    let metadata_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::metadata::PSP34MetadataDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::psp34::metadata::PSP34Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                pendzl::contracts::psp34::metadata::PSP34MetadataDefaultImpl::get_attribute_default_impl(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::metadata::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::psp34::metadata::PSP34MetadataData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PSP34Metadata", import);
    impl_args.imports.insert("PSP34MetadataData", import_data);

    impl_args.vec_import();

    override_functions("PSP34MetadataInternal", &mut internal, impl_args.map);
    override_functions("PSP34Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_default_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_ownable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::ownable::OwnableInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::ownable::OwnableInternal for #storage_struct_name {
            fn _owner(&self) -> Option<AccountId>{
                pendzl::contracts::ownable::OwnableInternalDefaultImpl::_owner_default_impl(self)
            }
            fn _update_owner(&mut self, owner: &Option<AccountId>){
                pendzl::contracts::ownable::OwnableInternalDefaultImpl::_update_owner_default_impl(self, owner);

            }
            fn _only_owner(&self) -> Result<(), OwnableError> {
                pendzl::contracts::ownable::OwnableInternalDefaultImpl::_only_owner_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let ownable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::ownable::OwnableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut ownable = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::ownable::Ownable for #storage_struct_name {
            #[ink(message)]
            fn owner(&self) -> Option<AccountId> {
                pendzl::contracts::ownable::OwnableDefaultImpl::owner_default_impl(self)
            }

            #[ink(message)]
            fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
                pendzl::contracts::ownable::OwnableDefaultImpl::renounce_ownership_default_impl(self)
            }

            #[ink(message)]
            fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
                pendzl::contracts::ownable::OwnableDefaultImpl::transfer_ownership_default_impl(self, new_owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::ownable::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::ownable::OwnableData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("Ownable", import);
    impl_args.imports.insert("OwnableData", import_data);

    override_functions("OwnableInternal", &mut internal, impl_args.map);
    override_functions("Ownable", &mut ownable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(ownable_default_impl));
    impl_args.items.push(syn::Item::Impl(ownable));
}

pub(crate) fn impl_access_control(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access_control::AccessControlInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access_control::AccessControlInternal for #storage_struct_name {
            fn _default_admin() -> RoleType {
                <Self as pendzl::contracts::access_control::AccessControlInternalDefaultImpl>::_default_admin_default_impl()
            }

            fn _has_role(&self, role: RoleType, account: Option<AccountId>) -> bool{
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_has_role_default_impl(self, role, account)
            }

            fn _grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_grant_role_default_impl(self, role, account)
            }

            fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>)  -> Result<(), AccessControlError>  {
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_do_revoke_role_default_impl(self, role, account)
            }

            fn _get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_get_role_admin_default_impl(self, role)
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_set_role_admin_default_impl(self, role, new_admin);
            }

            fn _ensure_has_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlInternalDefaultImpl::_ensure_has_role_default_impl(self, role, account)
            }

        }
    ))
    .expect("Should parse");

    let access_control_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access_control::AccessControlDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut access_control = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::access_control::AccessControl for #storage_struct_name {
            #[ink(message)]
            fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
                pendzl::contracts::access_control::AccessControlDefaultImpl::has_role_default_impl(self, role, address)
            }

            #[ink(message)]
            fn get_role_admin(&self, role: RoleType) -> RoleType {
                pendzl::contracts::access_control::AccessControlDefaultImpl::get_role_admin_default_impl(self, role)
            }

            #[ink(message)]
            fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlDefaultImpl::grant_role_default_impl(self, role, account)
            }

            #[ink(message)]
            fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlDefaultImpl::revoke_role_default_impl(self, role, account)
            }

            #[ink(message)]
            fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlDefaultImpl::renounce_role_default_impl(self, role, account)
            }

            #[ink(message)]
            fn set_role_admin(&mut self, role: RoleType, new_admin: RoleType) -> Result<(), AccessControlError> {
                pendzl::contracts::access_control::AccessControlDefaultImpl::set_role_admin_default_impl(self, role, new_admin)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::access_control::*;
    ))
    .expect("Should parse");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::access_control::AccessControlData;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("AccessControl", import);
    impl_args.imports.insert("AccessControlData", import_data);

    override_functions("AccessControlInternal", &mut internal, impl_args.map);
    override_functions("AccessControl", &mut access_control, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args
        .items
        .push(syn::Item::Impl(access_control_default_impl));
    impl_args.items.push(syn::Item::Impl(access_control));
}

pub(crate) fn impl_pausable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::pausable::PausableInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::pausable::PausableInternal for #storage_struct_name {
            fn _paused(&self) -> bool {
                pendzl::contracts::pausable::PausableInternalDefaultImpl::_paused_default_impl(self)
            }

            fn _pause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::pausable::PausableInternalDefaultImpl::_pause_default_impl(self)
            }

            fn _unpause(&mut self) -> Result<(), PausableError> {
                pendzl::contracts::pausable::PausableInternalDefaultImpl::_unpause_default_impl(self)
            }

            fn _ensure_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::pausable::PausableInternalDefaultImpl::_ensure_paused_default_impl(self)
            }

            fn _ensure_not_paused(&self) -> Result<(), PausableError> {
                pendzl::contracts::pausable::PausableInternalDefaultImpl::_ensure_not_paused_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let pausable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::pausable::PausableDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut pausable = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::pausable::Pausable for #storage_struct_name {
            #[ink(message)]
            fn paused(&self) -> bool {
                pendzl::contracts::pausable::PausableDefaultImpl::paused_default_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::pausable::*;
    ))
    .expect("Should parse import");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::pausable::PausableData;
    ))
    .expect("Should parse import");
    impl_args.imports.insert("Pausable", import);
    impl_args.imports.insert("PausableData", import_data);

    override_functions("PausableInternal", &mut internal, impl_args.map);
    override_functions("Pausable", &mut pausable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(pausable_default_impl));
    impl_args.items.push(syn::Item::Impl(pausable));
}
pub(crate) fn impl_vesting(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::general_vest::GeneralVestInternal for #storage_struct_name {
            fn _create_vest(&mut self,
                receiver: AccountId,
                asset: Option<AccountId>,
                amount: Balance,
                schedule: VestingSchedule,
                data: &Vec<u8>
            )-> Result<(), VestingError>  {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_create_vest_default_impl(
                    self,
                    receiver,
                    asset,
                    amount,
                    schedule,
                    data
                )
            }

            fn _release(&mut self, receiver: Option<AccountId>, asset: Option<AccountId>, data: &Vec<u8>) -> Result<u128, VestingError> {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_release_default_impl(self, receiver, asset, data)
            }

            fn _release_by_vest_id(&mut self, receiver: Option<AccountId>, asset: Option<AccountId>, id: u32, data: &Vec<u8>) -> Result<(), VestingError> {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_release_by_vest_id_default_impl(self, receiver, asset, id, data)
            }

            fn _handle_transfer_in(&mut self, asset: Option<AccountId>, from: AccountId, amount: Balance, data: &Vec<u8>) -> Result<(), VestingError> {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_handle_transfer_in_default_impl(self, asset, from, amount, data)
            }

            fn _handle_transfer_out(&mut self, asset: Option<AccountId>, to: AccountId, amount: Balance, data: &Vec<u8>) -> Result<(), VestingError> {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_handle_transfer_out_default_impl(self, asset, to, amount, data)
            }

            fn _next_id_vest_of(&self, of: AccountId, asset: Option<AccountId>, data: &Vec<u8>) -> u32 {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_next_id_vest_of_default_impl(self, of, asset, data)
            }

            fn _vesting_schedule_of(&self, of: AccountId, asset: Option<AccountId>, id: u32, data: &Vec<u8>) -> Option<VestingData> {
                pendzl::contracts::general_vest::GeneralVestInternalDefaultImpl::_vesting_schedule_of_default_impl(self, of, asset, id, data)
            }
        }
    ))
    .expect("Should parse");

    let vesting_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::general_vest::GeneralVestDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut general_vest = syn::parse2::<syn::ItemImpl>(quote!(
        impl  pendzl::contracts::general_vest::GeneralVest for #storage_struct_name {
            #[ink(message, payable)]
            fn create_vest(
                &mut self,
                receiver: AccountId,
                asset: Option<AccountId>,
                amount: Balance,
                schedule: VestingSchedule,
                data: Vec<u8>,
            ) -> Result<(), VestingError> {
                pendzl::contracts::general_vest::GeneralVestDefaultImpl::create_vest_default_impl(
                    self,
                    receiver,
                    asset,
                    amount,
                    schedule,
                    data
                )
            }
            #[ink(message)]
            fn release(&mut self, receiver: Option<AccountId>, asset: Option<AccountId>, data: Vec<u8>) -> Result<u128, VestingError> {
                pendzl::contracts::general_vest::GeneralVestDefaultImpl::release_default_impl(self, receiver, asset, data)
            }
            #[ink(message)]
            fn release_by_vest_id(&mut self, receiver: Option<AccountId>, asset: Option<AccountId>, id: u32, data: Vec<u8>) -> Result<(), VestingError> {
                pendzl::contracts::general_vest::GeneralVestDefaultImpl::release_by_vest_id_default_impl(self, receiver, asset, id, data)
            }
            #[ink(message)]
            fn next_id_vest_of(&self,  of: AccountId, asset: Option<AccountId>, data: Vec<u8>) -> u32 {
                pendzl::contracts::general_vest::GeneralVestDefaultImpl::next_id_vest_of_default_impl(self, of, asset, data)
            }
            #[ink(message)]
            fn vesting_schedule_of(&self, of: AccountId, asset: Option<AccountId>, id: u32, data: Vec<u8>) -> Option<VestingData> {
                pendzl::contracts::general_vest::GeneralVestDefaultImpl::vesting_schedule_of_default_impl(self, of, asset, id, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::general_vest::*;
    ))
    .expect("Should parse import");

    let import_data = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::general_vest::GeneralVestData;
    ))
    .expect("Should parse import");
    impl_args.imports.insert("GeneralVest", import);
    impl_args.imports.insert("GeneralVestData", import_data);

    impl_args.vec_import();

    override_functions("GeneralVestInternal", &mut internal, impl_args.map);
    override_functions("GeneralVest", &mut general_vest, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(vesting_default_impl));
    impl_args.items.push(syn::Item::Impl(general_vest));
}

pub(crate) fn impl_set_code_hash(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::set_code_hash::SetCodeHashInternalDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::set_code_hash::SetCodeHashInternal for #storage_struct_name {
            fn _set_code_hash(&mut self, set_code_hash: Hash) -> Result<(), SetCodeHashError> {
                pendzl::contracts::set_code_hash::SetCodeHashInternalDefaultImpl::_set_code_hash_default_impl(self, set_code_hash)
            }
        }
    ))
    .expect("Should parse");

    let upgradeable_default_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::set_code_hash::SetCodeHashDefaultImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut set_code_hash = syn::parse2::<syn::ItemImpl>(quote!(
        impl pendzl::contracts::set_code_hash::SetCodeHash for #storage_struct_name {
            #[ink(message)]
            fn set_code_hash(&mut self, set_code_hash: Hash) -> Result<(), SetCodeHashError> {
                pendzl::contracts::set_code_hash::SetCodeHashDefaultImpl::set_code_hash_default_impl(self, set_code_hash)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        pub use pendzl::contracts::set_code_hash::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("SetCodeHash", import);

    override_functions("SetCodeHashInternal", &mut internal, impl_args.map);
    override_functions("SetCodeHash", &mut set_code_hash, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_default_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args
        .items
        .push(syn::Item::Impl(upgradeable_default_impl));
    impl_args.items.push(syn::Item::Impl(set_code_hash));
}

fn override_functions(
    trait_name: &str,
    implementation: &mut syn::ItemImpl,
    map: &OverridenFnMap,
) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, inputs)) in overrides {
            let mut original_fn_found = false;
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        let args_diff = crate::internal::inputs_diff(
                            method.sig.inputs.clone(),
                            inputs.clone(),
                        );
                        if args_diff.added.len() > 0
                            || args_diff.removed.len() > 0
                        {
                            let original_args = method
                                .sig
                                .inputs
                                .clone()
                                .into_iter()
                                .map(|arg| {
                                    crate::internal::format_arg_string(
                                        &arg.into_token_stream().to_string(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            let current_args = inputs
                                .clone()
                                .into_iter()
                                .map(|arg| {
                                    crate::internal::format_arg_string(
                                        &arg.into_token_stream().to_string(),
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(", ");

                            panic!(
                                "Function arguments do not match for fn {} in trait {} \n
                            original args: {:?} \n
                            current args: {:?} \n
                            diff: {:?}",
                                fn_name, trait_name, original_args, current_args, args_diff
                            )
                        }

                        method.block = *fn_code.clone();
                        method.attrs.append(&mut attributes.to_vec());

                        original_fn_found = true;
                    }
                }
            }
            if !original_fn_found {
                panic!("Could not find fn {} in trait {}", fn_name, trait_name)
            }
        }
    }
}
