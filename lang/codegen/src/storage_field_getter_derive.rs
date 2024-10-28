// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, spanned::Spanned, Data};
/// Derives implementations of the `StorageFieldGetter` trait for fields annotated with `#[storage_field]`.
///
/// This function is intended to be used as a procedural macro for structs, enums, or unions. It scans the
/// data structure for fields annotated with the `#[storage_field]` attribute and generates implementations
/// of the `StorageFieldGetter` trait for the parent type, parameterized over the field's type. The trait
/// provides `get` and `get_mut` methods to access the annotated field.
///
/// # Arguments
///
/// * `item` - A `TokenStream` representing the input to the derive macro. This is typically the struct, enum,
///   or union on which the macro is invoked.
///
/// # Returns
///
/// * `TokenStream` - A token stream containing the generated implementations of `StorageFieldGetter` for
///   each annotated field.
///
/// # How It Works
///
/// 1. **Parsing the Input**:
///    - Parses the input `TokenStream` into a `syn::DeriveInput`, which represents the abstract syntax tree (AST)
///      of the input item (struct, enum, or union).
///    - Extracts the identifier (`struct_ident`) of the parent type and its generics.
///
/// 2. **Collecting Fields**:
///    - Collects all fields from the input item, regardless of whether it is a struct, enum, or union:
///      - For structs, collects the fields directly.
///      - For enums, collects fields from all variants.
///      - For unions, collects the named fields.
///
/// 3. **Filtering Annotated Fields**:
///    - Filters the collected fields to include only those that have the `#[storage_field]` attribute.
///
/// 4. **Generating Implementations**:
///    - For each annotated field:
///      - Extracts the field's identifier (`field_ident`), type (`ty`), and source code span (`span`).
///      - Generates an implementation of `StorageFieldGetter` for the parent type, parameterized over the field's type.
///      - The implementation provides:
///        - `fn get(&self) -> &FieldType` - Returns an immutable reference to the field.
///        - `fn get_mut(&mut self) -> &mut FieldType` - Returns a mutable reference to the field.
///      - Uses `quote_spanned!` to preserve the original source code span for better error messages.
///
/// 5. **Combining Implementations**:
///    - Collects all generated implementations into a single `TokenStream`.
///
/// 6. **Returning the Generated Code**:
///    - Wraps the collected implementations in a `quote!` block and returns the resulting `TokenStream`.
///
/// Given a struct with fields annotated by `#[storage_field]`:
///
/// ```rust
/// #[derive(StorageFieldGetter)]
/// pub struct MyStruct {
///     #[storage_field]
///     pub data: DataType,
///     pub other_field: u32,
/// }
/// ```
///
/// The macro will generate the following implementation:
///
/// ```rust
/// impl ::pendzl::traits::StorageFieldGetter<DataType> for MyStruct {
///     fn get(&self) -> &DataType {
///         &self.data
///     }
///
///     fn get_mut(&mut self) -> &mut DataType {
///         &mut self.data
///     }
/// }
/// ```
///
/// # Notes
///
/// - The macro supports structs, enums, and unions.
///   - For enums, it processes all variants and their fields.
///   - For unions, it processes the named fields.
/// - The `StorageFieldGetter` trait must be defined appropriately, with `get` and `get_mut` methods.
/// - The use of `quote_spanned!` ensures that any errors point back to the original source code location.
///
/// # Dependencies
///
/// - **Crates**:
///   - `syn`: For parsing Rust code into an AST.
///   - `quote`: For generating Rust code from the AST.
///   - `proc_macro2`: For handling token streams and spans in procedural macros.
pub fn storage_field_getter_derive(item: TokenStream) -> TokenStream {
    let derive: syn::DeriveInput = parse2(item).expect("Expected DeriveInput");

    let struct_ident = derive.ident;
    let (impls, types, where_clause) = derive.generics.split_for_impl();

    let fields: Vec<_> = match &derive.data {
        Data::Struct(st) => st.fields.iter().collect(),
        Data::Enum(en) => {
            en.variants.iter().flat_map(|v| v.fields.iter()).collect()
        }
        Data::Union(un) => un.fields.named.iter().collect(),
    };

    let impls = fields
        .iter()
        // filter out fields that don't have `#[storage_field]` attribute
        .filter(|field| field.attrs.iter().any(|a| a.path.is_ident("storage_field")))
        .map(|field| {
            let field_ident = field.ident.clone();
            // field type
            let ty = field.ty.clone();
            let span = field.span();

            quote::quote_spanned!(span=>
                impl #impls ::pendzl::traits::StorageFieldGetter<#ty> for #struct_ident #types #where_clause {
                    fn get(&self) -> &#ty {
                        &self.#field_ident
                    }

                    fn get_mut(&mut self) -> &mut #ty {
                        &mut self.#field_ident
                    }
                }
            )
        });

    quote! {
        #(#impls)*
    }
}
