// SPDX-License-Identifier: MIT
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

use pendzl_lang_codegen::{implementation, storage_derive, storage_item};
use proc_macro::TokenStream;

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

////This macro implements the default traits defined in pendzl, while also allowing users
////to override them with `#[overrider]` or `#[default_default_impl]` attributes. `#[overrider]` is used when
////you want to change the behavior of the method by your implementation. `#[default_default_impl]` is used when
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
////    // this will override a function from PSP22Internal
////    #[overrider(PSP22Internal)]
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
////    #[default_default_impl(PSP22)]
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
