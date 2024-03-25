// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

#![cfg_attr(not(feature = "std"), no_std, no_main)]

use pendzl_lang_codegen::{
    implementation, storage_field_getter_derive, storage_item,
};
use proc_macro::TokenStream;

/// The macro implements `pendzl::traits::StorageFieldGetter`
/// trait for each field marked by `#[storage_field]` attribute,
/// so it will be possible to access them via `self.data::<Type>()` method. It is mostly used for pendzl
/// to understand which fields should be accessed by traits.
///
/// # Example
/// ```skip
///     #[ink(storage)]
///     #[derive(StorageFieldGetter)]
///     pub struct Contract {
///         #[storage_field]
///         field: u32,
///     }
/// ```
#[proc_macro_derive(StorageFieldGetter, attributes(storage_field))]
pub fn storage_field_getter_derive(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    storage_field_getter_derive::storage_field_getter_derive(item.into()).into()
}

////This macro implements the default traits defined in pendzl, while also allowing users
////to override them with `#[overrider]` attribute. `#[overrider]` is used when
////you want to change the behavior of the method by your implementation.
////# Example
////```skip
////#[pendzl::implementation(PSP22)]
////#[ink::contract]
////pub mod MyInkToken {
////    use pendzl::traits::StorageFieldGetter;
////
////    #[ink(storage)]
////    #[derive(StorageFieldGetter)]
////    pub struct MyInkToken {
////        #[storage_field]
////        psp22: psp22::Data
////    }
////
////    // this will override a function from PSP22Internal
////    #[overrider(PSP22Internal)]
////    fn _update(
////        &mut self,
////        from: Option<&AccountId>,
////        to: Option<&AccountId>,
////        amount: &Balance,
////    ) -> Result<(), PSP22Error> {
////        // here we can change the update fn behavior
////    }
////
////    // this will override a function from PSP22
////    #[overrider(PSP22)]
////    fn balance_of(&self, owner: AccountId) -> Balance {
////         // here we can change the behavior of balance_of
////    }
//// ```
#[proc_macro_attribute]
pub fn implementation(
    attrs: TokenStream,
    ink_module: TokenStream,
) -> TokenStream {
    implementation::generate(attrs.into(), ink_module.into()).into()
}

synstructure::decl_attribute!(
    [storage_item] =>
    /// The macro implements `ink::storage_item` macro for the struct, which means that it prepares your struct
    /// to be a part of contract's storage. Also, inside of struct marked by this macro you can use
    /// `#[lazy]` attribute to mark fields, that should be lazily loaded and wrapped in `::ink::storage::Lazy`.
    /// The macro also generates constant storage keys for every mapping or lazy field and inserts them into
    /// type definition following recomendation from https://use.ink/datastructures/storage-layout
    ///
    /// # Example
    /// ```skip
    /// #[pendzl::storage_item]
    /// pub struct MyStruct {
    ///    a: u32,
    ///    b: u32,
    /// }
    /// ```
    ///
    /// # Example
    ///
    /// ```skip
    /// #[pendzl::storage_item]
    /// pub struct MyStruct {
    ///     #[lazy]
    ///     a: u32,
    ///     #[lazy]
    ///     b: u32,
    /// }
    ///
    /// # Example
    ///
    /// ```skip
    /// #[pendzl::storage_item]
    /// pub struct MyStruct {
    ///     field: u32
    ///     #[lazy]
    ///     lazy_field: u32,
    ///     mappping: Mapping<AccountId, u128>,
    /// }
    /// ```
    /// will be transformed into:
    /// ```skip
    /// #[::ink::storage_item]
    /// pub struct MyStruct {
    ///    field: u32
    ///    lazy_field: ::ink::storage::Lazy<u32, ::ink::storage::traits::ManualKey<MYSTRUCT_LAZY_FIELD_STORAGE_KEY>>,
    ///    mappping: ::ink::storage::Lazy<Mapping<AccountId, u128>, ::ink::storage::traits::ManualKey<MYSTRUCT_MAPPING_STORAGE_KEY>>,
    /// }
    /// pub const MYSTRUCT_LAZY_FIELD_STORAGE_KEY: u32 = ::pendzl::storage_unique_key!(MyStruct, lazy_field);
    /// pub const MYSTRUCT_MAPPING_STORAGE_KEY: u32 = ::pendzl::storage_unique_key!(MyStruct, mapping);
    /// ```
    storage_item::storage_item
);
