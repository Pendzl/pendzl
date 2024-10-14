// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, Field, Fields};

use crate::internal::is_attr;

/// Processes struct fields to generate storage keys for fields annotated with `#[lazy]`
/// and fields of type `Mapping`, based on the struct name and field name.
///
/// This function transforms fields in a struct to include manual storage keys, which are necessary
/// for deterministic storage layouts in smart contracts. It handles fields annotated with `#[lazy]`
/// and fields of type `Mapping`, generating unique storage keys for them and modifying their types
/// accordingly. Other fields are returned unchanged.
///
/// # Functionality
///
/// - **Fields with `#[lazy]` Attribute**:
///   - Wraps the field's type with `::ink::storage::Lazy<...>` using a manually specified storage key.
///   - Generates a unique storage key constant for the field.
///   - Removes the `#[lazy]` attribute from the field's attributes.
///   - Example transformation:
///     ```rust
///     #[lazy]
///     pub my_lazy_field: u32,
///     // Becomes:
///     pub my_lazy_field: ::ink::storage::Lazy<u32, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_LAZY_FIELD>>,
///     ```
///
/// - **Fields of Type `Mapping`**:
///   - Modifies the `Mapping` type to include `::ink::storage::traits::ManualKey<...>` with a generated storage key constant.
///   - Generates a unique storage key constant for the field.
///   - Example transformation:
///     ```rust
///     pub my_mapping: Mapping<AccountId, u128>,
///     // Becomes:
///     pub my_mapping: Mapping<AccountId, u128, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_MAPPING>>,
///     ```
///
/// - **Other Fields**:
///   - Leaves the field unchanged.
///   - Example:
///     ```rust
///     pub my_field: u32,
///     // Remains:
///     pub my_field: u32,
///     ```
///
/// # Arguments
///
/// * `structure_name` - The name of the struct containing the fields. Used to generate unique storage key names.
/// * `fields` - The fields of the struct to process, represented as a `syn::Fields` object.
///
/// # Returns
///
/// A tuple containing:
///
/// - `Vec<Field>`: A vector of transformed `Field` objects. Fields are modified based on whether they
///   have the `#[lazy]` attribute or are of type `Mapping`.
/// - `Vec<Option<TokenStream>>`: A vector of optional `TokenStream`s representing generated storage key constants.
///   Each element corresponds to a field. If a storage key was generated for a field, its `Option<TokenStream>`
///   will be `Some(token_stream)`, otherwise `None`.
///
/// # Detailed Explanation
///
/// The function operates by mapping over each field and applying transformations based on the field's attributes and type.
///
/// ## Processing Fields with `#[lazy]` Attribute
///
/// 1. **Clone the Field**: To avoid mutating the original field in the syntax tree, the field is cloned.
/// 2. **Extract Field Information**:
///    - **Type (`ty`)**: The field's type is extracted and converted to a token stream.
///    - **Span (`span`)**: The field's type span is captured for use in code generation.
///    - **Field Name (`field_name`)**: The field's identifier is unwrapped and converted to a string.
/// 3. **Generate Storage Key Name (`key_name`)**:
///    - Creates a unique constant name by combining the struct name and field name in uppercase.
///    - Example: For struct `MyStruct` and field `my_field`, the key name would be `STORAGE_KEY_MYSTRUCT_MY_FIELD`.
/// 4. **Create Storage Key Constant**:
///    - Uses the `::pendzl::storage_unique_key!` macro to generate a unique storage key value.
///    - Generates code like:
///      ```rust
///      pub const STORAGE_KEY_MYSTRUCT_MY_FIELD: u32 = ::pendzl::storage_unique_key!("MyStruct", "my_field");
///      ```
/// 5. **Modify Field Type**:
///    - Wraps the original field type with `::ink::storage::Lazy`, including the manual storage key.
///    - Uses `quote_spanned!` to preserve the original span for accurate error reporting.
///    - The modified type becomes:
///      ```rust
///      ::ink::storage::Lazy<OriginalType, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_FIELD>>
///      ```
/// 6. **Remove `#[lazy]` Attribute**:
///    - Filters out the `#[lazy]` attribute from the field's attributes to prevent it from appearing in the generated code.
/// 7. **Return Modified Field and Storage Key**:
///    - Returns the transformed field and the generated storage key token stream wrapped in `Some`.
///
/// ## Processing Fields of Type `Mapping`
///
/// 1. **Clone the Field**: The field is cloned to avoid mutating the original.
/// 2. **Extract Field Information**:
///    - **Span (`span`)**: Captures the field's type span.
///    - **Field Name (`field_name`)**: Converts the field's identifier to a string.
/// 3. **Determine if Field is a `Mapping`**:
///    - Checks if the field's type is a path type (`syn::Type::Path`).
///    - Examines the last segment of the path to see if it is `Mapping`.
/// 4. **Generate Storage Key (if `Mapping`)**:
///    - If the field is of type `Mapping`, generates a unique storage key name as before.
///    - Creates a storage key constant similar to the `#[lazy]` case.
/// 5. **Modify `Mapping` Type**:
///    - Modifies the `Mapping` type to include the manual storage key as an additional generic argument.
///    - For example:
///      ```rust
///      Mapping<Key, Value>
///      // Becomes:
///      Mapping<Key, Value, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_FIELD>>
///      ```
///    - Handles complex types by adjusting the second generic argument (index 1) to include the storage key.
/// 6. **Return Modified Field and Storage Key**:
///    - Returns the transformed field and the generated storage key token stream wrapped in `Some`.
///
/// ## Processing Other Fields
///
/// - The field is returned unchanged.
/// - No storage key is generated (the corresponding `Option<TokenStream>` is `None`).
///
/// ## Collecting Results
///
/// - Uses `unzip()` to separate the transformed fields and the optional storage keys into two vectors.
/// - The transformed fields can be used to reconstruct the struct with the updated types.
/// - The storage keys can be inserted into the code to define the constants.
///
/// # Example Usage
///
/// Given a struct:
///
/// ```rust
/// pub struct MyStruct {
///     pub my_field: u32,
///     #[lazy]
///     pub my_lazy_field: u32,
///     pub my_mapping: Mapping<AccountId, u128>,
/// }
/// ```
///
/// After processing, the transformed fields and storage keys would be:
///
/// **Transformed Fields:**
///
/// ```rust
/// pub my_field: u32,
/// pub my_lazy_field: ::ink::storage::Lazy<u32, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_LAZY_FIELD>>,
/// pub my_mapping: Mapping<AccountId, u128, ::ink::storage::traits::ManualKey<STORAGE_KEY_MYSTRUCT_MY_MAPPING>>,
/// ```
///
/// **Generated Storage Keys:**
///
/// ```rust
/// pub const STORAGE_KEY_MYSTRUCT_MY_LAZY_FIELD: u32 = ::pendzl::storage_unique_key!("MyStruct", "my_lazy_field");
/// pub const STORAGE_KEY_MYSTRUCT_MY_MAPPING: u32 = ::pendzl::storage_unique_key!("MyStruct", "my_mapping");
/// ```
///
/// # Dependencies and Helper Functions
///
/// - **Crates**:
///   - `syn`: For parsing and manipulating Rust syntax trees.
///   - `quote`: For generating code as token streams.
///   - `proc_macro2`: For handling spans and tokens in procedural macros.
/// - **Helper Function**:
///   - `is_attr`: Checks if a field has a specific attribute. It might be implemented as:
///     ```rust
///     fn is_attr(attrs: &[syn::Attribute], ident: &str) -> bool {
///         attrs.iter().any(|attr| attr.path.is_ident(ident))
///     }
///     ```
///
/// # Important Notes
///
/// - **Field Identifiers**: The function assumes all fields have identifiers (named fields). It will panic if used with unnamed fields (tuple structs).
/// - **Unique Storage Keys**: Generates unique storage keys to prevent conflicts and ensure deterministic storage layout.
/// - **Attribute Consumption**: Removes the `#[lazy]` attribute after processing to prevent duplication or unintended behavior.
///
/// # Potential Errors and Panics
///
/// - **Missing Field Identifier**: If a field does not have an identifier (e.g., in a tuple struct), the function will panic when calling `field.ident.as_ref().unwrap()`.
/// - **Type Parsing**: If the field's type does not match expected patterns (e.g., unexpected generic structures), the function may not correctly modify the type.
///
fn generate_manual_keys_for_fields(
    structure_name: &str,
    fields: Fields,
) -> (Vec<Field>, Vec<Option<TokenStream>>) {
    fields
        .iter()
        .map(|field| {
            if is_attr(&field.attrs, "lazy") {
                let mut new_field = field.clone();
                let ty = field.ty.clone().to_token_stream();
                let span = field.ty.span();
                let field_name = field.ident.as_ref().unwrap().to_string();

                let key_name = format_ident!(
                    "STORAGE_KEY_{}_{}",
                    structure_name.to_uppercase(),
                    field_name.to_uppercase()
                );

                // generate code for storage key to be unique and not default to AutoKey
                let storage_key = quote! {
                    pub const #key_name: u32 = ::pendzl::storage_unique_key!(#structure_name, #field_name);
                };

                // use generated store key in the field
                new_field.ty = syn::Type::Verbatim(quote_spanned!(span =>
                    ::ink::storage::Lazy<#ty, ::ink::storage::traits::ManualKey<#key_name>>
                ));

                // consume lazy attribute
                new_field.attrs = field
                    .attrs
                    .iter()
                    .filter(|attr| !attr.path.is_ident("lazy"))
                    .cloned()
                    .collect();

                (new_field, Some(storage_key))
            } else {
                let mut new_field = field.clone();
                let span = field.ty.span();
                let field_name = field.ident.as_ref().unwrap().to_string();

                let key_name = format_ident!(
                    "STORAGE_KEY_{}_{}",
                    structure_name.to_uppercase(),
                    field_name.to_uppercase()
                );

                let is_mapping = if let syn::Type::Path(path) = &field.ty {
                    if let Some(segment) = path.path.segments.last() {
                        segment.ident == "Mapping"
                    } else {
                        false
                    }
                } else {
                    false
                };

                // generate code for storage key to be unique and not default to AutoKey
                let storage_key = if is_mapping {
                    Some(quote! {
                        pub const #key_name: u32 = ::pendzl::storage_unique_key!(#structure_name, #field_name);
                    })
                } else {
                    None
                };

                // Mapping<AccountId, u128>
                // -> Mapping<AccountId, u128, ::ink::storage::traits::ManualKey<STORAGE_KEY_...>)>

                // Mapping<(AccountId, Option<AccountId>, u32), SomeStruct>
                // -> Mapping<(AccountId, Option<AccountId>, u32), SomeStruct, ::ink::storage::traits::ManualKey<STORAGE_KEY_...>)>
                if let syn::Type::Path(path) = &mut new_field.ty {
                    if let Some(segment) = path.path.segments.last_mut() {
                        if segment.ident == "Mapping" {
                            let mut args = segment.arguments.clone();
                            if let syn::PathArguments::AngleBracketed(args) = &mut args {
                                if let Some(syn::GenericArgument::Type(ty)) = args.args.iter_mut().nth(1) {
                                    *ty = syn::Type::Verbatim(quote_spanned!(span =>
                                        #ty, ::ink::storage::traits::ManualKey<#key_name>
                                    ));
                                }
                            }
                            segment.arguments = args;
                        }
                    }
                }

                (new_field, storage_key)
            }
        })
        .unzip()
}

/// Generates code for a struct, including its fields and associated storage keys.
///
/// This function processes a struct's syntax tree to generate the corresponding Rust code
/// along with any necessary storage keys for the struct's fields. It handles structs with
/// named fields, unnamed fields (tuple structs), and includes any attributes, generics,
/// and where clauses associated with the struct.
///
/// # Arguments
///
/// * `s` - A reference to a `synstructure::Structure` representing the syntax tree of the struct.
/// * `struct_item` - A `DataStruct` containing the struct's data, including its fields.
///
/// # Returns
///
/// * `TokenStream` - A token stream containing the generated code for the struct and its storage keys.
///
/// # How It Works
///
/// 1. **Extract Struct Information**:
///    - Clones the struct's identifier (name), visibility, attributes, and generics from the syntax tree.
///    - Splits the generics to capture any `where` clauses.
///
/// 2. **Generate Fields and Storage Keys**:
///    - Calls `generate_manual_keys_for_fields` to generate field declarations and storage keys.
///    - This function handles both named and unnamed fields and returns:
///        - `fields`: A vector of field declarations.
///        - `storage_keys`: A vector of optional `TokenStream`s representing storage keys.
///
/// 3. **Determine Struct Type**:
///    - Uses a `match` statement on `struct_item.fields` to check if the struct has unnamed fields (tuple struct) or named fields.
///
/// 4. **Generate Final Code**:
///    - Constructs the struct definition accordingly:
///        - For unnamed fields (tuple structs), uses parentheses `()` around the fields.
///        - For named fields, uses braces `{}` around the fields.
///    - Includes:
///        - Attributes (`attrs`).
///        - Visibility (`vis`).
///        - Struct name (`struct_ident`).
///        - Generics (`types`).
///        - Where clauses (`where_closure`).
///        - Fields (`fields`).
///    - Appends any generated storage keys after the struct definition.
///
/// The function will handle the unnamed fields and generate the appropriate code.
fn generate_struct(
    s: &synstructure::Structure,
    struct_item: DataStruct,
) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let types = s.ast().generics.clone();
    let attrs = s.ast().attrs.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let (fields, storage_keys) = generate_manual_keys_for_fields(
        struct_ident.to_string().as_str(),
        struct_item.fields.clone(),
    );

    match struct_item.fields {
        Fields::Unnamed(_) => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure (
                    #(#fields),*
                );

                #(#storage_keys)*
            }
        }
        _ => {
            quote! {
                #(#attrs)*
                #vis struct #struct_ident #types #where_closure {
                    #(#fields),*
                }

                #(#storage_keys)*
            }
        }
    }
}

/// Generates code for an enum, including its variants and associated storage keys.
///
/// This function processes an enum's syntax tree to generate the corresponding Rust code
/// along with any necessary storage keys for the enum's fields. It handles enums with
/// unit variants, named fields, unnamed fields, and explicit discriminants.
///
/// # Arguments
///
/// * `s` - A reference to a `synstructure::Structure` representing the syntax tree of the enum.
/// * `enum_item` - A `DataEnum` containing the enum's data, including its variants.
///
/// # Returns
///
/// * `TokenStream` - A token stream containing the generated code for the enum and its storage keys.
///
/// # How It Works
///
/// 1. **Extract Enum Information**: Clones the enum's identifier, visibility, attributes, and generics from the syntax tree.
/// 2. **Initialize Storage Keys Vector**: Prepares a vector to collect storage keys for the enum's fields.
/// 3. **Process Each Variant**:
///    - Handles explicit discriminants if present.
///    - Generates fields and storage keys for each variant.
///    - Formats the fields based on whether they are named, unnamed, or unit variants.
///    - Collects storage keys for deferred code generation.
/// 4. **Generate Final Code**:
///    - Constructs the enum definition with all its variants.
///    - Includes any attributes, generics, and where clauses.
///    - Appends generated storage keys after the enum definition.
///
/// The function will generate code that properly defines the enum with its variants,
/// handles explicit discriminants, and generates storage keys for its fields.
fn generate_enum(
    s: &synstructure::Structure,
    enum_item: DataEnum,
) -> TokenStream {
    let enum_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();
    let mut all_storage_keys: Vec<Option<TokenStream>> = vec![];

    let variants = enum_item.variants.into_iter().map(|variant| {
        let attrs = variant.attrs;
        let variant_ident = &variant.ident;

        // handle explicit discriminants, ex. `ExUnNamed(bool) = 123,`
        let discriminant = if let Some((eq, expr)) = variant.discriminant {
            quote! { #eq #expr}
        } else {
            quote! {}
        };

        // get wrapped variant fields & keys - handles both unit, named (ExNamed{a: bool}) or unnamed (ExNamed(bool))
        let (fields, storage_keys) = generate_manual_keys_for_fields(
            format!("{}_{}", enum_ident, variant_ident).as_str(),
            variant.fields.clone(),
        );

        let fields = match variant.fields {
            Fields::Named(_) => quote! { { #(#fields),* } },
            Fields::Unnamed(_) => quote! { ( #(#fields),* ) },
            Fields::Unit => quote! {},
        };

        // Collect the storage keys generated for this variant's fields.
        // Defer generating code for storage keys until after the enum is generated.
        all_storage_keys.extend(storage_keys);

        // generate code
        quote! {
            #(#attrs)*
            #variant_ident #fields #discriminant,
        }
    });

    // output atrributes, types and possible where of the enum, generated variant keys & all of the storage keys
    quote! {
        #(#attrs)*
        #vis enum #enum_ident #types #where_closure {
            #(#variants),*
        }

        #(#all_storage_keys)*
    }
}

/// This function implements `ink::storage_item` macro for the struct, which means that it prepares your struct
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
pub fn storage_item(
    _attrs: TokenStream,
    s: synstructure::Structure,
) -> TokenStream {
    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        Data::Enum(enum_item) => generate_enum(&s, enum_item),
        Data::Union(_) => panic!(
            "{} - pendzl storage_item cannot wrap Union",
            s.ast().ident.clone()
        ),
    };

    quote! {
        #[::ink::storage_item]
        #item
    }
}
