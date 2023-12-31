use quote::{
    format_ident,
    quote,
};
use std::collections::HashMap;
use syn::Block;

pub type IsDefault = bool;
pub type OverridenFnMap = HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

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

    fn signature_import(&mut self) {
        let sig_import = syn::parse2::<syn::ItemUse>(quote!(
            use pendzl::utils::crypto::Signature;
        ))
        .expect("Should parse");
        self.imports.insert("Signature", sig_import);
    }
}

pub(crate) fn impl_psp22(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp22::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
                psp22::InternalImpl::_emit_transfer_event_impl(self, from, to, amount)
            }

            fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
                psp22::InternalImpl::_emit_approval_event_impl(self, owner, spender, amount)
            }

            fn _total_supply(&self) -> Balance {
                psp22::InternalImpl::_total_supply_impl(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                psp22::InternalImpl::_balance_of_impl(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                psp22::InternalImpl::_allowance_impl(self, owner, spender)
            }

            fn _transfer_from_to(
                &mut self,
                from: AccountId,
                to: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_transfer_from_to_impl(self, from, to, amount, data)
            }

            fn _approve_from_to(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                amount: Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_approve_from_to_impl(self, owner, spender, amount)
            }

            fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_mint_to_impl(self, account, amount)
            }

            fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_burn_from_impl(self, account, amount)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_before_token_transfer_impl(self, from, to, amount)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), PSP22Error> {
                psp22::InternalImpl::_after_token_transfer_impl(self, from, to, amount)
            }
        }
    ))
    .expect("Should parse");

    let psp22_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22 for #storage_struct_name {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                PSP22Impl::total_supply_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                PSP22Impl::balance_of_impl(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                PSP22Impl::allowance_impl(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error> {
                PSP22Impl::transfer_impl(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP22Error> {
                PSP22Impl::transfer_from_impl(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::approve_impl(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::increase_allowance_impl(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), PSP22Error> {
                PSP22Impl::decrease_allowance_impl(self, spender, delta_value)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22", import);
    impl_args.vec_import();

    override_functions("psp22::Internal", &mut internal, impl_args.map);
    override_functions("PSP22", &mut psp22, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp22_impl));
    impl_args.items.push(syn::Item::Impl(psp22));
}

pub(crate) fn impl_psp22_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22MintableImpl::mint_impl(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22Mintable", import);
    impl_args.vec_import();

    override_functions("PSP22Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp22_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22BurnableImpl::burn_impl(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22Burnable", import);
    impl_args.vec_import();

    override_functions("PSP22Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp22_permit(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let permit_internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl permit::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let permit_internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl permit::Internal for #storage_struct_name {
            fn _permit(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                amount: Balance,
                deadline: u64,
                signature: Signature,
            ) -> Result<(), PSP22Error> {
                permit::InternalImpl::_permit_impl(self, owner, spender, amount, deadline, signature)
            }
            fn _domain_separator(&mut self) -> [u8; 32] {
                permit::InternalImpl::_domain_separator_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let permit_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl permit::PSP22PermitImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let permit = syn::parse2::<syn::ItemImpl>(quote!(
        impl permit::PSP22Permit for #storage_struct_name {
            #[ink(message)]
            fn permit(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                value: Balance,
                deadline: u64,
                signature: Signature,
            ) -> Result<(), PSP22Error> {
                permit::PSP22PermitImpl::permit_impl(self, owner, spender, value, deadline, signature)
            }

            #[ink(message)]
            fn domain_separator(&mut self) -> [u8; 32] {
                permit::PSP22PermitImpl::domain_separator_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::permit::*;
    ))
    .expect("Should parse");

    impl_args.imports.insert("PSP22Permit", import);
    impl_args.signature_import();
    impl_args.vec_import();

    // TODO
    // override_functions("PSP22Permit", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(permit_internal_impl));
    impl_args.items.push(syn::Item::Impl(permit_internal));

    impl_args.items.push(syn::Item::Impl(permit_impl));
    impl_args.items.push(syn::Item::Impl(permit));
}

pub(crate) fn impl_psp22_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Metadata for #storage_struct_name {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                PSP22MetadataImpl::token_name_impl(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                PSP22MetadataImpl::token_symbol_impl(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                PSP22MetadataImpl::token_decimals_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22Metadata", import);
    impl_args.vec_import();

    override_functions("PSP22Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp22_capped(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::Internal for #storage_struct_name {
            fn _init_cap(&mut self, cap: Balance) -> Result<(), PSP22Error> {
                capped::InternalImpl::_init_cap_impl(self, cap)
            }

            fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
                capped::InternalImpl::_is_cap_exceeded_impl(self, amount)
            }

            fn _cap(&self) -> Balance {
                capped::InternalImpl::_cap_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let capped_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22CappedImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut capped = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Capped for #storage_struct_name {
            #[ink(message)]
            fn cap(&self) -> Balance {
                PSP22CappedImpl::cap_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::capped::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22Capped", import);
    impl_args.vec_import();

    override_functions("capped::Internal", &mut internal, impl_args.map);
    override_functions("PSP22Capped", &mut capped, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(capped_impl));
    impl_args.items.push(syn::Item::Impl(capped));
}

pub(crate) fn impl_psp22_wrapper(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::Internal for #storage_struct_name {
            fn _recover(&mut self, account: AccountId) -> Result<Balance, PSP22Error> {
                wrapper::InternalImpl::_recover_impl(self, account)
            }

            fn _deposit(&mut self, amount: Balance) -> Result<(), PSP22Error> {
                wrapper::InternalImpl::_deposit_impl(self, amount)
            }

            fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                wrapper::InternalImpl::_withdraw_impl(self, account, amount)
            }

            fn _underlying_balance(&mut self) -> Balance {
                wrapper::InternalImpl::_underlying_balance_impl(self)
            }

            fn _init(&mut self, underlying: AccountId) {
                wrapper::InternalImpl::_init_impl(self, underlying)
            }

            fn _underlying(&mut self) -> Option<AccountId> {
                wrapper::InternalImpl::_underlying_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let wrapper_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22WrapperImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut wrapper = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22Wrapper for #storage_struct_name {
            #[ink(message)]
            fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22WrapperImpl::deposit_for_impl(self, account, amount)
            }

            #[ink(message)]
            fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
                PSP22WrapperImpl::withdraw_to_impl(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::wrapper::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22Wrapper", import);
    impl_args.vec_import();

    override_functions("wrapper::Internal", &mut internal, impl_args.map);
    override_functions("PSP22Wrapper", &mut wrapper, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(wrapper_impl));
    impl_args.items.push(syn::Item::Impl(wrapper));
}

pub(crate) fn impl_flashmint(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::Internal for #storage_struct_name {
            fn _get_fee(&self, amount: Balance) -> Balance {
                flashmint::InternalImpl::_get_fee_impl(self, amount)
            }

            fn _on_flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                fee: Balance,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                flashmint::InternalImpl::_on_flashloan_impl(self, receiver_account, token, fee, amount, data)
            }
        }
    ))
    .expect("Should parse");

    let flashlender_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLenderImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut flashlender = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLender for #storage_struct_name {
            #[ink(message)]
            fn max_flashloan(&mut self, token: AccountId) -> Balance {
                FlashLenderImpl::max_flashloan_impl(self, token)
            }

            #[ink(message)]
            fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
                FlashLenderImpl::flash_fee_impl(self, token, amount)
            }

            #[ink(message)]
            fn flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                FlashLenderImpl::flashloan_impl(self, receiver_account, token, amount, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::extensions::flashmint::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("Flashmint", import);
    impl_args.vec_import();

    override_functions("flashmint::Internal", &mut internal, impl_args.map);
    override_functions("FlashLender", &mut flashlender, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(flashlender_impl));
    impl_args.items.push(syn::Item::Impl(flashlender));
}

pub(crate) fn impl_token_timelock(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::Internal for #storage_struct_name {
            fn _withdraw(&mut self, amount: Balance) -> Result<(), PSP22TokenTimelockError> {
                token_timelock::InternalImpl::_withdraw_impl(self, amount)
            }

            fn _contract_balance(&mut self) -> Balance {
                token_timelock::InternalImpl::_contract_balance_impl(self)
            }

            fn _init(
                &mut self,
                token: AccountId,
                beneficiary: AccountId,
                release_time: Timestamp,
            ) -> Result<(), PSP22TokenTimelockError> {
                token_timelock::InternalImpl::_init_impl(self, token, beneficiary, release_time)
            }

            fn _token(&self) -> Option<AccountId> {
                token_timelock::InternalImpl::_token_impl(self)
            }

            fn _beneficiary(&self) -> Option<AccountId> {
                token_timelock::InternalImpl::_beneficiary_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let timelock_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22TokenTimelockImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut timelock = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP22TokenTimelock for #storage_struct_name {
            #[ink(message)]
            fn token(&self) -> Option<AccountId> {
                PSP22TokenTimelockImpl::token_impl(self)
            }

            #[ink(message)]
            fn beneficiary(&self) -> Option<AccountId> {
                PSP22TokenTimelockImpl::beneficiary_impl(self)
            }

            #[ink(message)]
            fn release_time(&self) -> Timestamp {
                PSP22TokenTimelockImpl::release_time_impl(self)
            }

            #[ink(message)]
            fn release(&mut self) -> Result<(), PSP22TokenTimelockError> {
                PSP22TokenTimelockImpl::release_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp22::utils::token_timelock::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP22TokenTimelock", import);

    override_functions("token_timelock::Internal", &mut internal, impl_args.map);
    override_functions("PSP22TokenTimelock", &mut timelock, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(timelock_impl));
    impl_args.items.push(syn::Item::Impl(timelock));
}

pub(crate) fn impl_psp34(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
                psp34::InternalImpl::_emit_transfer_event_impl(self, from, to, id)
            }

            fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
                psp34::InternalImpl::_emit_approval_event_impl(self, from, to, id, approved)
            }

            fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_approve_for_impl(self, to, id, approved)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                psp34::InternalImpl::_owner_of_impl(self, id)
            }

            fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_transfer_token_impl(self, to, id, data)
            }

            fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_mint_to_impl(self, to, id)
            }

            fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_burn_from_impl(self, from, id)
            }

            fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
                psp34::InternalImpl::_allowance_impl(self, owner, operator, id)
            }

            fn _check_token_exists(&self, id: &Id) -> Result<AccountId, PSP34Error> {
                psp34::InternalImpl::_check_token_exists_impl(self, id)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_before_token_transfer_impl(self, from, to, id)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), PSP34Error> {
                psp34::InternalImpl::_after_token_transfer_impl(self, from, to, id)
            }
        }
    ))
    .expect("Should parse");

    let psp34_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34 for #storage_struct_name {
            #[ink(message)]
            fn collection_id(&self) -> Id {
                PSP34Impl::collection_id_impl(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> u32 {
                PSP34Impl::balance_of_impl(self, owner)
            }

            #[ink(message)]
            fn owner_of(&self, id: Id) -> Option<AccountId> {
                PSP34Impl::owner_of_impl(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
                PSP34Impl::allowance_impl(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), PSP34Error> {
                PSP34Impl::approve_impl(self, operator, id, approved)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error> {
                PSP34Impl::transfer_impl(self, to, id, data)
            }

            #[ink(message)]
            fn total_supply(&self) -> Balance {
                PSP34Impl::total_supply_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let psp34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                psp34::BalancesManagerImpl::_balance_of_impl(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                psp34::BalancesManagerImpl::_increase_balance_impl(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                psp34::BalancesManagerImpl::_decrease_balance_impl(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                psp34::BalancesManagerImpl::_total_supply_impl(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                psp34::BalancesManagerImpl::_owner_of_impl(self, id)
            }

            fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()> {
                psp34::BalancesManagerImpl::_operator_approvals_impl(self, owner, operator, id)
            }

            fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                psp34::BalancesManagerImpl::_insert_operator_approvals_impl(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                psp34::BalancesManagerImpl::_remove_operator_approvals_impl(self, owner, operator, id)
            }

            fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
                psp34::BalancesManagerImpl::_insert_token_owner_impl(self, id, to)
            }

            fn _remove_token_owner(&mut self, id: &Id) {
                psp34::BalancesManagerImpl::_remove_token_owner_impl(self, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp34::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP34", import);
    impl_args.vec_import();

    override_functions("psp34::BalancesManager", &mut psp34_balances, impl_args.map);
    override_functions("psp34::Internal", &mut internal, impl_args.map);
    override_functions("PSP34", &mut psp34, impl_args.map);

    // only insert this if it is not present
    impl_args
        .overriden_traits
        .entry("psp34::BalancesManager")
        .or_insert(syn::Item::Impl(psp34_balances));

    impl_args
        .overriden_traits
        .entry("psp34::BalancesManagerImpl")
        .or_insert(syn::Item::Impl(psp34_balances_impl));

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp34_impl));
    impl_args.items.push(syn::Item::Impl(psp34));
}

pub(crate) fn impl_psp34_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
                PSP34BurnableImpl::burn_impl(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp34::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP34Burnable", import);
    impl_args.vec_import();

    override_functions("PSP34Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp34_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
                PSP34MintableImpl::mint_impl(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp34::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP34Mintable", import);
    impl_args.vec_import();

    override_functions("PSP34Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp34_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for #storage_struct_name {
            fn _emit_attribute_set_event(&self, id: Id, key: String, data: String) {
                metadata::InternalImpl::_emit_attribute_set_event_impl(self, id, key, data)
            }

            fn _set_attribute(&mut self, id: Id, key: String, value: String) {
                metadata::InternalImpl::_set_attribute_impl(self, id, key, value)
            }
        }
    ))
    .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                PSP34MetadataImpl::get_attribute_impl(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp34::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP34Metadata", import);
    impl_args.vec_import();

    override_functions("metadata::Internal", &mut internal, impl_args.map);
    override_functions("PSP34Mintable", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp34_enumerable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34EnumerableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP34Enumerable for #storage_struct_name {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error> {
                PSP34EnumerableImpl::owners_token_by_index_impl(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error> {
                PSP34EnumerableImpl::token_by_index_impl(self, index)
            }
        }

    ))
    .expect("Should parse");

    let psp34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp34::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                enumerable::BalancesManagerImpl::_balance_of_impl(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                enumerable::BalancesManagerImpl::_increase_balance_impl(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                enumerable::BalancesManagerImpl::_decrease_balance_impl(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                enumerable::BalancesManagerImpl::_total_supply_impl(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                enumerable::BalancesManagerImpl::_owner_of_impl(self, id)
            }

            fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()> {
                enumerable::BalancesManagerImpl::_operator_approvals_impl(self, owner, operator, id)
            }

            fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                enumerable::BalancesManagerImpl::_insert_operator_approvals_impl(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                enumerable::BalancesManagerImpl::_remove_operator_approvals_impl(self, owner, operator, id)
            }

            fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
                enumerable::BalancesManagerImpl::_insert_token_owner_impl(self, id, to)
            }

            fn _remove_token_owner(&mut self, id: &Id) {
                enumerable::BalancesManagerImpl::_remove_token_owner_impl(self, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp34::extensions::enumerable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP34Enumerable", import);
    impl_args.vec_import();

    override_functions("psp34::BalancesManager", &mut psp34_balances, impl_args.map);
    override_functions("PSP34Enumerable", &mut psp34_enumerable, impl_args.map);

    impl_args
        .overriden_traits
        .insert("psp34::BalancesManager", syn::Item::Impl(psp34_balances));
    impl_args
        .overriden_traits
        .insert("psp34::BalancesManagerImpl", syn::Item::Impl(psp34_balances_impl));

    impl_args.items.push(syn::Item::Impl(enumerable_impl));
    impl_args.items.push(syn::Item::Impl(psp34_enumerable));
}

pub(crate) fn impl_psp37(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id, amount: Balance) {
                psp37::InternalImpl::_emit_transfer_event_impl(self, from, to, id, amount)
            }

            fn _emit_transfer_batch_event(
                &self,
                from: Option<AccountId>,
                to: Option<AccountId>,
                ids_amounts: Vec<(Id, Balance)>,
            ) {
                psp37::InternalImpl::_emit_transfer_batch_event_impl(self, from, to, ids_amounts)
            }

            fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, value: Balance) {
                psp37::InternalImpl::_emit_approval_event_impl(self, owner, operator, id, value)
            }

            fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_mint_to_impl(self, to, ids_amounts)
            }

            fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_burn_from_impl(self, from, ids_amounts)
            }

            fn _transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_transfer_from_impl(self, from, to, id, amount, data)
            }

            fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
                psp37::InternalImpl::_get_allowance_impl(self, account, operator, id)
            }

            fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_approve_for_impl(self, operator, id, value)
            }

            fn _decrease_allowance(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Id,
                value: Balance,
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_decrease_allowance_impl(self, owner, operator, id, value)
            }

            fn _transfer_token(
                &mut self,
                from: &AccountId,
                to: &AccountId,
                id: Id,
                amount: Balance,
                data: &[u8],
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_transfer_token_impl(self, from, to, id, amount, data)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &[(Id, Balance)],
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_before_token_transfer_impl(self, from, to, ids)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &[(Id, Balance)],
            ) -> Result<(), PSP37Error> {
                psp37::InternalImpl::_after_token_transfer_impl(self, from, to, ids)
            }
        }

    ))
    .expect("Should parse");

    let psp37_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp37 = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37 for #storage_struct_name {
            #[ink(message)]
            fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
                PSP37Impl::balance_of_impl(self, owner, id)
            }

            #[ink(message)]
            fn total_supply(&self, id: Option<Id>) -> Balance {
                PSP37Impl::total_supply_impl(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
                PSP37Impl::allowance_impl(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), PSP37Error> {
                PSP37Impl::approve_impl(self, operator, id, value)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), PSP37Error> {
                PSP37Impl::transfer_impl(self, to, id, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37Impl::transfer_from_impl(self, from, to, id, value, data)
            }
        }
    ))
    .expect("Should parse");

    let psp37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                psp37::BalancesManagerImpl::_balance_of_impl(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                psp37::BalancesManagerImpl::_total_supply_impl(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), PSP37Error> {
                psp37::BalancesManagerImpl::_increase_balance_impl(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), PSP37Error> {
                psp37::BalancesManagerImpl::_decrease_balance_impl(self, owner, id, amount, burn)
            }

            fn _insert_operator_approvals(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Option<&Id>,
                amount: &Balance,
            ) {
                psp37::BalancesManagerImpl::_insert_operator_approvals_impl(self, owner, operator, id, amount)
            }

            fn _get_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Option<Balance> {
                psp37::BalancesManagerImpl::_get_operator_approvals_impl(self, owner, operator, id)
            }
            fn _remove_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) {
                psp37::BalancesManagerImpl::_remove_operator_approvals_impl(self, owner, operator, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37", import);
    impl_args.vec_import();

    override_functions("psp37::BalancesManager", &mut psp37_balances, impl_args.map);
    override_functions("psp37::Internal", &mut internal, impl_args.map);
    override_functions("PSP37", &mut psp37, impl_args.map);

    // only insert this if it is not present
    impl_args
        .overriden_traits
        .entry("psp37::BalancesManager")
        .or_insert(syn::Item::Impl(psp37_balances));

    impl_args
        .overriden_traits
        .entry("psp37::BalancesManagerImpl")
        .or_insert(syn::Item::Impl(psp37_balances_impl));

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(psp37_impl));
    impl_args.items.push(syn::Item::Impl(psp37));
}

pub(crate) fn impl_psp37_batch(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::Internal for #storage_struct_name {
            fn _batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                batch::InternalImpl::_batch_transfer_from_impl(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let batch_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37BatchImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut batch = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Batch for #storage_struct_name {
            #[ink(message)]
            fn batch_transfer(
                &mut self,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37BatchImpl::batch_transfer_impl(self, to, ids_amounts, data)
            }

            #[ink(message)]
            fn batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), PSP37Error> {
                PSP37BatchImpl::batch_transfer_from_impl(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::extensions::batch::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37Batch", import);
    impl_args.vec_import();

    override_functions("batch::Internal", &mut internal, impl_args.map);
    override_functions("PSP37Batch", &mut batch, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(batch_impl));
    impl_args.items.push(syn::Item::Impl(batch));
}

pub(crate) fn impl_psp37_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                PSP37BurnableImpl::burn_impl(self, from, ids_amounts)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37Burnable", import);
    impl_args.vec_import();

    override_functions("PSP37Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_psp37_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for #storage_struct_name {
            fn _emit_attribute_set_event(&self, id: &Id, key: &String, data: &String) {
                metadata::InternalImpl::_emit_attribute_set_event_impl(self, id, key, data);
            }

            fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), PSP37Error> {
                metadata::InternalImpl::_set_attribute_impl(self, id, key, data)
            }

            fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
                metadata::InternalImpl::_get_attribute_impl(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                PSP37MetadataImpl::get_attribute_impl(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37Metadata", import);
    impl_args.vec_import();

    override_functions("metadata::Internal", &mut internal, impl_args.map);
    override_functions("PSP37Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_psp37_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
                PSP37MintableImpl::mint_impl(self, to, ids_amounts)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37Mintable", import);
    impl_args.vec_import();

    override_functions("PSP37Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_psp37_enumerable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37EnumerableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp37_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl PSP37Enumerable for #storage_struct_name {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
                PSP37EnumerableImpl::owners_token_by_index_impl(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Option<Id> {
                PSP37EnumerableImpl::token_by_index_impl(self, index)
            }
        }
    ))
    .expect("Should parse");

    let psp37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut psp37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl psp37::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_balance_of_impl(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_total_supply_impl(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), PSP37Error> {
                enumerable::BalancesManagerImpl::_increase_balance_impl(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), PSP37Error> {
                enumerable::BalancesManagerImpl::_decrease_balance_impl(self, owner, id, amount, burn)
            }

            fn _insert_operator_approvals(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Option<&Id>,
                amount: &Balance,
            ) {
                enumerable::BalancesManagerImpl::_insert_operator_approvals_impl(self, owner, operator, id, amount)
            }

            fn _get_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Option<Balance> {
                enumerable::BalancesManagerImpl::_get_operator_approvals_impl(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>){
                enumerable::BalancesManagerImpl::_remove_operator_approvals_impl(self, owner, operator, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::psp37::extensions::enumerable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("PSP37Enumerable", import);
    impl_args.vec_import();

    override_functions("psp37::BalancesManager", &mut psp37_balances, impl_args.map);
    override_functions("PSP37Enumerable", &mut psp37_enumerable, impl_args.map);

    impl_args
        .overriden_traits
        .insert("psp37::BalancesManager", syn::Item::Impl(psp37_balances));
    impl_args
        .overriden_traits
        .insert("psp37::BalancesManagerImpl", syn::Item::Impl(psp37_balances_impl));

    impl_args.items.push(syn::Item::Impl(enumerable_impl));
    impl_args.items.push(syn::Item::Impl(psp37_enumerable));
}

pub(crate) fn impl_ownable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl ownable::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl ownable::Internal for #storage_struct_name {
            fn _emit_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>) {
                ownable::InternalImpl::_emit_ownership_transferred_event_impl(self, previous, new)
            }

            fn _init_with_owner(&mut self, owner: AccountId) {
                ownable::InternalImpl::_init_with_owner_impl(self, owner)
            }
            fn _only_owner(&self) -> Result<(), OwnableError> {
                ownable::InternalImpl::_only_owner_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let ownable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl OwnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut ownable = syn::parse2::<syn::ItemImpl>(quote!(
        impl Ownable for #storage_struct_name {
            #[ink(message)]
            fn owner(&self) -> Option<AccountId> {
                OwnableImpl::owner_impl(self)
            }

            #[ink(message)]
            fn renounce_ownership(&mut self) -> Result<(), OwnableError> {
                OwnableImpl::renounce_ownership_impl(self)
            }

            #[ink(message)]
            fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), OwnableError> {
                OwnableImpl::transfer_ownership_impl(self, new_owner)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::ownable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("Ownable", import);

    override_functions("ownable::Internal", &mut internal, impl_args.map);
    override_functions("Ownable", &mut ownable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(ownable_impl));
    impl_args.items.push(syn::Item::Impl(ownable));
}

pub(crate) fn impl_access_control(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl access_control::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl access_control::Internal for #storage_struct_name {
            fn _emit_role_admin_changed(&mut self, role: RoleType, previous: RoleType, new: RoleType) {
                access_control::InternalImpl::_emit_role_admin_changed_impl(self, role, previous, new);
            }

            fn _emit_role_granted(&mut self, role: RoleType, grantee: Option<AccountId>, grantor: Option<AccountId>) {
                access_control::InternalImpl::_emit_role_granted_impl(self, role, grantee, grantor);
            }

            fn _emit_role_revoked(&mut self, role: RoleType, account: Option<AccountId>, sender: AccountId) {
                access_control::InternalImpl::_emit_role_revoked_impl(self, role, account, sender);
            }

            fn _default_admin() -> RoleType {
                <Self as access_control::InternalImpl>::_default_admin_impl()
            }

            fn _init_with_caller(&mut self) {
                access_control::InternalImpl::_init_with_caller_impl(self);
            }

            fn _init_with_admin(&mut self, admin: Option<AccountId>) {
                access_control::InternalImpl::_init_with_admin_impl(self, admin);
            }

            fn _setup_role(&mut self, role: RoleType, member: Option<AccountId>) {
                access_control::InternalImpl::_setup_role_impl(self, role, member);
            }

            fn _do_revoke_role(&mut self, role: RoleType, account: Option<AccountId>) {
                access_control::InternalImpl::_do_revoke_role_impl(self, role, account);
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                access_control::InternalImpl::_set_role_admin_impl(self, role, new_admin);
            }

            fn _ensure_has_role(&self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                access_control::InternalImpl::_ensure_has_role_impl(self, role, account)
            }

            fn _get_role_admin(&self, role: RoleType) -> RoleType {
                access_control::InternalImpl::_get_role_admin_impl(self, role)
            }
        }
    ))
    .expect("Should parse");

    let access_control_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AccessControlImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut access_control = syn::parse2::<syn::ItemImpl>(quote!(
        impl AccessControl for #storage_struct_name {
            #[ink(message)]
            fn has_role(&self, role: RoleType, address: Option<AccountId>) -> bool {
                AccessControlImpl::has_role_impl(self, role, address)
            }

            #[ink(message)]
            fn get_role_admin(&self, role: RoleType) -> RoleType {
                AccessControlImpl::get_role_admin_impl(self, role)
            }

            #[ink(message)]
            fn grant_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                AccessControlImpl::grant_role_impl(self, role, account)
            }

            #[ink(message)]
            fn revoke_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                AccessControlImpl::revoke_role_impl(self, role, account)
            }

            #[ink(message)]
            fn renounce_role(&mut self, role: RoleType, account: Option<AccountId>) -> Result<(), AccessControlError> {
                AccessControlImpl::renounce_role_impl(self, role, account)
            }
        }
    ))
    .expect("Should parse");

    let members_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl access_control::MembersManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut members = syn::parse2::<syn::ItemImpl>(quote!(
        impl access_control::MembersManager for #storage_struct_name {
            fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
                access_control::MembersManagerImpl::_has_role_impl(self, role, address)
            }

            fn _add(&mut self, role: RoleType, member: &Option<AccountId>) {
                access_control::MembersManagerImpl::_add_impl(self, role, member)
            }

            fn _remove(&mut self, role: RoleType, member: &Option<AccountId>) {
                access_control::MembersManagerImpl::_remove_impl(self, role, member)
            }

            fn _get_role_admin(&self, role: RoleType) -> Option<RoleType> {
                access_control::MembersManagerImpl::_get_role_admin_impl(self, role)
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                access_control::MembersManagerImpl::_set_role_admin_impl(self, role, new_admin)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access_control::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AccessControl", import);

    override_functions("access_control::MembersManager", &mut members, impl_args.map);
    override_functions("access_control::Internal", &mut internal, impl_args.map);
    override_functions("AccessControl", &mut access_control, impl_args.map);

    // only insert these if it is not present
    impl_args
        .overriden_traits
        .entry("access_control::MembersManagerImpl")
        .or_insert(syn::Item::Impl(members_impl));

    impl_args
        .overriden_traits
        .entry("access_control::MembersManager")
        .or_insert(syn::Item::Impl(members));

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(access_control_impl));
    impl_args.items.push(syn::Item::Impl(access_control));
}

pub(crate) fn impl_access_control_enumerable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AccessControlEnumerableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AccessControlEnumerable for #storage_struct_name {
            #[ink(message)]
            fn get_role_member(&self, role: RoleType, index: u32) -> Option<AccountId> {
                AccessControlEnumerableImpl::get_role_member_impl(self, role, index)
            }

            #[ink(message)]
            fn get_role_member_count(&self, role: RoleType) -> u32 {
                AccessControlEnumerableImpl::get_role_member_count_impl(self, role)
            }
        }
    ))
    .expect("Should parse");

    let members_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::MembersManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut members = syn::parse2::<syn::ItemImpl>(quote!(
        impl access_control::MembersManager for #storage_struct_name {
            fn _has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
                enumerable::MembersManagerImpl::_has_role_impl(self, role, address)
            }

            fn _add(&mut self, role: RoleType, member: &Option<AccountId>) {
                enumerable::MembersManagerImpl::_add_impl(self, role, member)
            }

            fn _remove(&mut self, role: RoleType, member: &Option<AccountId>) {
                enumerable::MembersManagerImpl::_remove_impl(self, role, member)
            }

            fn _get_role_admin(&self, role: RoleType) -> Option<RoleType> {
                enumerable::MembersManagerImpl::_get_role_admin_impl(self, role)
            }

            fn _set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
                enumerable::MembersManagerImpl::_set_role_admin_impl(self, role, new_admin)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::access_control::extensions::enumerable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AccessControlEnumerable", import);

    override_functions("access_control::MembersManager", &mut members, impl_args.map);
    override_functions("AccessControlEnumerable", &mut enumerable, impl_args.map);

    impl_args
        .overriden_traits
        .insert("access_control::MembersManagerImpl", syn::Item::Impl(members_impl));
    impl_args
        .overriden_traits
        .insert("access_control::MembersManager", syn::Item::Impl(members));

    impl_args.items.push(syn::Item::Impl(enumerable_impl));
    impl_args.items.push(syn::Item::Impl(enumerable));
}

pub(crate) fn impl_pausable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl pausable::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl pausable::Internal for #storage_struct_name {
            fn _emit_paused_event(&self, account: AccountId) {
                pausable::InternalImpl::_emit_paused_event_impl(self, account)
            }

            fn _emit_unpaused_event(&self, account: AccountId) {
                pausable::InternalImpl::_emit_unpaused_event_impl(self, account)
            }

            fn _paused(&self) -> bool {
                pausable::InternalImpl::_paused_impl(self)
            }

            fn _pause(&mut self) -> Result<(), PausableError> {
                pausable::InternalImpl::_pause_impl(self)
            }

            fn _unpause(&mut self) -> Result<(), PausableError> {
                pausable::InternalImpl::_unpause_impl(self)
            }

            fn _switch_pause(&mut self) -> Result<(), PausableError> {
                pausable::InternalImpl::_switch_pause_impl(self)
            }

            fn _ensure_paused(&self) -> Result<(), PausableError> {
                pausable::InternalImpl::_ensure_paused_impl(self)
            }

            fn _ensure_not_paused(&self) -> Result<(), PausableError> {
                pausable::InternalImpl::_ensure_not_paused_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let pausable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl PausableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut pausable = syn::parse2::<syn::ItemImpl>(quote!(
        impl Pausable for #storage_struct_name {
            #[ink(message)]
            fn paused(&self) -> bool {
                PausableImpl::paused_impl(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::pausable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("Pausable", import);

    override_functions("pausable::Internal", &mut internal, impl_args.map);
    override_functions("Pausable", &mut pausable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(pausable_impl));
    impl_args.items.push(syn::Item::Impl(pausable));
}

pub(crate) fn impl_nonces(impl_args: &mut ImplArgs) {
    let storage_struct_name = &impl_args.contract_name();

    let nonces_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl NoncesImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let nonces = syn::parse2::<syn::ItemImpl>(quote!(
        impl Nonces for #storage_struct_name {
            #[ink(message)]
            fn nonces(&self, account: AccountId) -> u64 {
                NoncesImpl::nonces_impl(self, &account)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::contracts::nonces::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("Nonces", import);

    impl_args.items.push(syn::Item::Impl(nonces_impl));
    impl_args.items.push(syn::Item::Impl(nonces));
}
fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}
