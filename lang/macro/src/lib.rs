// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![cfg_attr(not(feature = "std"), no_std, no_main)]

// use proc_macro::TokenStream;

use pendzl_lang_codegen::{
    // trait_definition,
    // wrapper,
    accessors,
    // contract,
    implementation,
    // modifier_definition,
    // modifiers,
    storage_derive,
    storage_item,
};
use proc_macro::TokenStream;

/// Entry point for use pendzl's macros in ink! smart contracts.
///
/// # Description
///
/// The macro consumes pendzl's macros to simplify the usage of the library.
/// After consumption, it pastes ink! code and then ink!'s macros will be processed.
///
/// This macro consumes impl section for traits defined with [`#[ink::trait_definition]`](`macro@crate::trait_definition`).
// #[proc_macro_attribute]
// pub fn contract(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
//     contract::generate(_attrs.into(), ink_module.into()).into()
// }

/// Defines extensible trait in the scope of `pendzl::contract`.
/// It is a common rust trait, so you can use any features of rust inside of this trait.
/// If this trait contains some methods marked with `#[ink(message)]` or `#[ink(constructor)]` attributes,
/// this macro will extract these attributes and will put them into a separate trait
/// (the separate trait only is used to call methods from the original trait), but the macro will not touch methods.
///
/// This macro stores definition of the trait in a temporary file during build process.
/// Based on this definition [`#[ink::contract]`](`macro@crate::contract`)
/// will generate implementation of additional traits.
///
///  ** Note ** The name of the trait defined via this macro must be unique for the whole project.
///  ** Note ** You can't use aliases, generics, and other rust's stuff in signatures of ink!'s methods.
///
/// # Example: Definition
///
/// ```
/// mod doc {
/// use ink::storage::Mapping;
/// use pendzl::traits::{ AccountId, Balance, Storage };
///
/// #[derive(Debug)]
/// #[ink::storage_item]
/// pub struct Data {
///     pub balances: Mapping<AccountId, Balance>,
/// }
///
/// #[ink::trait_definition]
/// pub trait PSP22: Storage<Data> {
///     /// Returns the account Balance for the specified `owner`.
///     #[ink(message)]
///     fn balance_of(&self, owner: AccountId) -> Balance {
///         self.data().balances.get(&owner).unwrap_or(0)
///     }
///
///     /// Transfers `value` amount of tokens from the caller's account to account `to`.
///     #[ink(message)]
///     fn transfer(&mut self, to: AccountId, value: Balance) {
///         self._transfer_from_to(to, to, value)
///     }
///
///     fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///         let from_balance = self.balance_of(from);
///         assert!(from_balance >= amount, "InsufficientBalance");
///         self.data().balances.insert(from, &(from_balance - amount));
///         let to_balance = self.balance_of(to);
///         self.data().balances.insert(to, &(to_balance + amount));
///     }
/// }
/// }
/// ```
///
/// # Example: Implementation
///
/// ```
/// #[ink::contract]
/// mod base_psp22 {
///     use ink::storage::traits::ManualKey;
///     use ink::storage::Mapping;
///     use ink::storage::Lazy;
///     use pendzl::traits::Storage;
///
///     const STORAGE_KEY_1: u32 = 101;
///     const STORAGE_KEY_2: u32 = 102;
///     const STORAGE_KEY_3: u32 = 103;
///
///     #[derive(Default, Debug)]
///     #[ink::storage_item]
///     pub struct Data {
///         pub supply: Lazy<Balance, ManualKey<STORAGE_KEY_1>>,
///         pub balances: Mapping<AccountId, Balance, ManualKey<STORAGE_KEY_2>>,
///         pub allowances: Mapping<(AccountId, AccountId), Balance, ManualKey<STORAGE_KEY_3>>,
///     }
///
///     #[ink::trait_definition]
///     pub trait PSP22Example: Storage<Data> {
///         /// Returns the account Balance for the specified `owner`.
///         #[ink(message)]
///         fn balance_of(&self, owner: AccountId) -> Balance {
///             self.data().balances.get(&owner).unwrap_or(0)
///         }
///
///         /// Transfers `value` amount of tokens from the caller's account to account `to`.
///         #[ink(message)]
///         fn transfer(&mut self, to: AccountId, value: Balance) {
///             let from = Self::env().caller();
///             self._transfer_from_to(from, to, value)
///         }
///
///         fn _transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Balance) {
///             let from_balance = self.balance_of(from);
///             assert!(from_balance >= amount, "InsufficientBalance");
///             self.data().balances.insert(from, &(from_balance - amount));
///             let to_balance = self.balance_of(to);
///             self.data().balances.insert(to, &(to_balance + amount));
///         }
///     }
///
///     #[ink(storage)]
///     #[derive(Storage, Default)]
///     pub struct PSP22Struct {
///         #[storage_field]
///         example: Data,
///         hated_account: Option<AccountId>,
///     }
///
///     impl PSP22Example for PSP22Struct {}
///
///     impl PSP22Struct {
///         #[ink(constructor)]
///         pub fn new(hated_account: AccountId) -> Self {
///             let mut instance = Self::default();
///             instance.hated_account = Some(hated_account);
///             instance
///         }
///     }
/// }
/// The macro implements `pendzl::traits::Storage`
/// trait for each field marked by `#[storage_field]` attribute,
/// so it will be possible to access them via `self.data::<Type>()` method. It is mostly used for pendzl
/// to understand which fields should be accessed by traits.
///
/// # Example
/// ```skip
///     #[ink(storage)]
///     #[derive(Storage)]
///     pub struct Contract {
///         #[storage_field]
///         field: u32,
///     }
/// ```
#[proc_macro_derive(Storage, attributes(storage_field))]
pub fn storage_derive(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    storage_derive::storage_derive(item.into()).into()
}

synstructure::decl_attribute!(
    [accessors] =>
    ////Macro that automatically implements accessors like get/set for struct fields, that implements `scale::Encode`
    ////and `scale::Decode` traits. You should specify the getters trait naming in the macro's attribute.
    ////Also, fields that you want getters to be generated, should be marked by `#[get]` attribute.
    ////Fields, that you want setters to be generated, should be marked by `#[set]` attribute.
    ////The name of the accessor message will be concatenation of `get/set` + `_` + field's name.
    ////
    ////# Example:
    ////```skip
    ////
    ////use pendzl::traits::Storage;
    ////
    ////#[pendzl::accessors(SomeStructGetters)]
    ////#[derive(Default)]
    ////#[ink::storage_item]
    ////pub struct SomeStruct {
    ////    #[get]
    ////    a: u32,
    ////    b: u32,
    ////    #[set]
    ////    c: u32,
    ////}
    ////
    ////#[ink::contract]
    ////pub mod contract {
    ////    use crate::*;
    ////    use pendzl::traits::Storage;
    ////
    ////    #[ink(storage)]
    ////    #[derive(Storage, Default)]
    ////    pub struct Contract {
    ////        #[storage_field]
    ////        some_struct: SomeStruct,
    ////    }
    ////
    ////    impl SomeStructGetters for Contract {}
    ////
    ////    impl Contract {
    ////        #[ink(constructor)]
    ////        pub fn new() -> Self {
    ////            Self::default()
    ////        }
    ////    }
    ////}
    ////```
    accessors::accessors
);

////This macro implements the default traits defined in pendzl, while also allowing users
////to override them with `#[overrider]` or `#[default_impl]` attributes. `#[overrider]` is used when
////you want to change the behavior of the method by your implementation. `#[default_impl]` is used when
////you want to keep the default implementation from pendzl, but you want to attach some modifiers to
////that function.
////# Example
////```skip
////#[pendzl::implementation(PSP22)]
////#[ink::contract]
////pub mod MyInkToken {
////    use pendzl::traits::Storage;
////
////    #[ink(storage)]
////    #[derive(Storage)]
////    pub struct MyInkToken {
////        #[storage_field]
////        psp22: psp22::Data
////    }
////
////    // this will override a function from psp22::Internal
////    #[overrider(psp22::Internal)]
////    fn _before_token_transfer(
////        &mut self,
////        from: Option<&AccountId>,
////        to: Option<&AccountId>,
////        amount: &Balance,
////    ) -> Result<(), PSP22Error> {
////        // here we can change the behavior before token transfer
////    }
////
////    // this will override a function from PSP22
////    #[overrider(PSP22)]
////    fn balance_of(&self, owner: AccountId) -> Balance {
////         // here we can change the behavior of balance_of
////    }
////
////    // this will keep the default implementation of this method,
////    // however, it will add the modifier (and possibly other attributes defined by user)
////    // to the function. In this case, we don't even have to worry about the attributes and
////    // return type of the function
////    #[default_impl(PSP22)]
////    #[modifiers(...)]
////    fn transfer_from() {}
////
////    impl Contract {
////        // we can add constructor and other messages
////    }
//// }
//// ```
#[proc_macro_attribute]
pub fn implementation(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    implementation::generate(attrs.into(), ink_module.into()).into()
}

synstructure::decl_attribute!(
    [storage_item] =>
    /// The macro implements `ink::storage_item` macro for the struct, which means that it prepares your struct
    /// to be a part of contract's storage. Also, inside of struct marked by this macro you can use
    /// `#[lazy]` attribute to mark fields, that should be lazily loaded and wrapped in `::ink::storage::Lazy`.
    /// The macro also generates constant storage keys for every mapping or lazy field and inserts them into
    /// type definition.
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
    storage_item::storage_item
);
