// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::{
    implementations::*,
    internal::{is_attr, AttributeArgs},
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{Item, Path};

pub fn generate(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    let input: TokenStream = ink_module;

    // map attribute args to provide default impls
    let to_inject_default_impls_vec = syn::parse2::<AttributeArgs>(attrs)
        .expect("No traits to provide default impls for provided")
        .iter()
        .map(|method| method.to_token_stream().to_string().replace(' ', ""))
        .collect::<Vec<String>>();

    let mut module = syn::parse2::<syn::ItemMod>(input)
        .expect("Can't parse contract module");
    let (braces, items) = match module.clone().content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line pendzl modules are not supported, use `#[implementation] mod name {{ ... }}`",
            )
        }
    };

    // name of struct for which we will implement the traits
    let ident = extract_storage_struct_name(&items);
    // we will look for overriden functions and remove them from the mod
    let (map, mut items) = consume_overriders(items);

    // to save importing of stuff by users
    let mut imports = HashMap::<&str, syn::ItemUse>::default();
    // if multiple contracts are using the same trait implemented differently we override it this way
    let mut overriden_traits = HashMap::<&str, syn::Item>::default();

    let mut impl_args = ImplArgs::new(
        &map,
        &mut items,
        &mut imports,
        &mut overriden_traits,
        ident,
    );

    for to_inject_default_impls in &to_inject_default_impls_vec {
        match to_inject_default_impls.as_str() {
            "PSP22" => impl_psp22(&mut impl_args),
            "PSP22Burnable" => impl_psp22_burnable(&mut impl_args),
            "PSP22Mintable" => impl_psp22_mintable(&mut impl_args),
            "PSP22Vault" => impl_psp22_vault(&mut impl_args),
            "PSP22Metadata" => impl_psp22_metadata(&mut impl_args),
            "PSP34" => impl_psp34(&mut impl_args),
            "PSP34Burnable" => impl_psp34_burnable(&mut impl_args),
            "PSP34Metadata" => impl_psp34_metadata(&mut impl_args),
            "PSP34Mintable" => impl_psp34_mintable(&mut impl_args),
            "Ownable" => impl_ownable(&mut impl_args),
            "AccessControl" => impl_access_control(&mut impl_args),
            "Pausable" => impl_pausable(&mut impl_args),
            "GeneralVest" => impl_vesting(&mut impl_args),
            "SetCodeHash" => impl_set_code_hash(&mut impl_args),
            _ => panic!("pendzl::implementation({to_inject_default_impls}) not implemented!"),
        }
    }

    cleanup_imports(impl_args.imports);

    let import_storage = syn::parse2::<syn::ItemUse>(quote!(
        use pendzl::traits::StorageFieldGetter;
    ))
    .expect("Should parse import");

    impl_args.imports.insert("PendzlStorage", import_storage);
    // add the imports
    impl_args.items.append(
        &mut impl_args
            .imports
            .values()
            .cloned()
            .map(syn::Item::Use)
            .collect(),
    );

    // add overriden traits
    impl_args
        .items
        .append(&mut impl_args.overriden_traits.values().cloned().collect());

    module.content = Some((braces, items));

    quote! {
        #module
    }
}

/// Cleans up the `imports` map by removing unnecessary base trait imports when extended traits are present.
///
/// This function modifies the `imports` map, which contains imports required for the contract.
/// It checks for the presence of extended trait imports and removes the corresponding base trait import
/// to prevent redundant or conflicting imports in the generated code. This optimization ensures that
/// only the necessary imports are included avoiding potential import conflicts.
///
fn cleanup_imports(imports: &mut HashMap<&str, syn::ItemUse>) {
    fn check_and_remove_import(
        name_to_check: &str,
        to_check: Vec<&str>,
        imports: &mut HashMap<&str, syn::ItemUse>,
    ) {
        if to_check.iter().any(|name| imports.contains_key(name)) {
            imports.remove(name_to_check);
        }
    }

    // we will remove unnecessary imports
    let psp22_default_impls = vec![
        "PSP22Mintable",
        "PSP22Burnable",
        "PSP22Metadata",
        "PSP22Vault",
    ];
    check_and_remove_import("PSP22", psp22_default_impls, imports);

    let psp34_default_impls =
        vec!["PSP34Mintable", "PSP34Burnable", "PSP34Metadata"];
    check_and_remove_import("PSP34", psp34_default_impls, imports);
}

/// Extracts and removes functions annotated with `#[overrider(trait_name)]` from a list of items,
/// mapping them for later use and returning the remaining items. Later used in `override_functions` fn.
///
/// This function processes a vector of `syn::Item`, typically representing the items in a Rust module.
/// It searches for functions annotated with `#[overrider(trait_name)]`, which are intended to override
/// default implementations of trait methods. These functions are extracted, their attributes and signatures
/// are processed, and they are stored in an `OverridenFnMap` for later use in code generation or trait
/// implementation overrides. The annotated functions are removed from the original items to prevent duplicate
/// definitions. The function returns a tuple containing the map of overridden functions and the remaining items.
///
/// # Arguments
///
/// * `items` - A vector of `syn::Item` representing the items within a Rust module.
///
/// # Returns
///
/// A tuple containing:
///
/// - `OverridenFnMap`: A `HashMap` where each key is a trait name (`String`), and the value is a vector of
///   tuples representing the overridden functions for that trait. Each tuple contains:
///     - The function name (`String`).
///     - A tuple with:
///         - The function body (`Box<Block>`).
///         - A vector of attributes (`Vec<syn::Attribute>`), excluding the `#[overrider]` attribute.
///         - The function's input arguments (`Punctuated<FnArg, Comma>`).
/// - `Vec<syn::Item>`: A vector of the remaining `syn::Item` after removing the overridden functions.
///
/// # Panics
///
/// - If the `#[overrider]` attribute is found but does not contain a trait name, the function will panic with
///   `"Expected overridden trait identifier"`.
/// - If the function cannot find the `#[overrider]` attribute in the attributes list when attempting to remove it,
///   it will panic with `"No {attr_name} attribute found!"`.
///
fn consume_overriders(
    items: Vec<syn::Item>,
) -> (OverridenFnMap, Vec<syn::Item>) {
    // Initialize an empty HashMap to store overridden functions.
    let mut map = HashMap::new();
    // Initialize a vector to collect items that are not overridden functions.
    let mut result: Vec<syn::Item> = vec![];

    // Iterate over each item in the input `items`.
    items.into_iter().for_each(|mut item| {
        // Check if the item is a function (`Item::Fn`).
        if let Item::Fn(item_fn) = &mut item {
            // Check if the function has the `#[overrider(trait_name)]` attribute.
            if is_attr(&item_fn.attrs, "overrider") {
                let attr_name = "overrider";
                // Extract the function name as a string.
                let fn_name = item_fn.sig.ident.to_string();
                // Clone the function's code block.
                let code = item_fn.block.clone();
                // Clone the function's attributes.
                let mut attributes = item_fn.attrs.clone();
                // Clone the function's input arguments.
                let inputs = item_fn.sig.inputs.clone();

                // Remove the `#[overrider]` attribute from the attributes list to prevent it from appearing in the generated code.
                let to_remove_idx = attributes
                    .iter()
                    .position(|attr| is_attr(&[attr.clone()], attr_name))
                    .expect("No {attr_name} attribute found!");
                let overrider_attribute = attributes.remove(to_remove_idx);

                // Parse the trait name from the attribute's arguments.
                let trait_name = overrider_attribute
                    .parse_args::<Path>()
                    .expect("Expected overridden trait identifier")
                    .to_token_stream()
                    .to_string();

                // Retrieve the existing vector of functions for the trait or create a new one.
                let mut vec = map.get(&trait_name).unwrap_or(&vec![]).clone();
                // Add the overridden function to the vector.
                vec.push((fn_name, (code, attributes, inputs)));
                // Insert the updated vector back into the map under the trait name.
                map.insert(trait_name, vec.to_vec());
            } else {
                // If not an overridden function, add it to the result vector.
                result.push(item);
            }
        } else {
            // Non-function items are added to the result vector unchanged.
            result.push(item);
        }
    });

    // Return the map of overridden functions and the remaining items.
    (map, result)
}

/// Extracts the name of the contract's storage struct from a list of syntax items.
///
/// This function iterates over the provided `items`, which are syntax representations
/// of Rust items (such as structs, enums, functions, etc.), and searches for a struct
/// annotated with `#[ink(storage)]`. The `#[ink(storage)]` attribute designates the
/// storage struct in an ink! smart contract.
///
/// Once the storage struct is found, the function returns its identifier (name) as a `String`.
///
/// # Arguments
///
/// * `items` - A slice of `syn::Item` representing the items within a Rust module.
///
/// # Returns
///
/// * A `String` containing the name of the storage struct.
///
/// # Panics
///
/// * If no struct annotated with `#[ink(storage)]` is found within the provided items,
///   the function will panic with the message `"Contract storage struct not found!"`.
///
/// # Example
///
/// ```rust
/// use syn::{Item, parse_quote};
///
/// // Suppose we have the following struct in our code:
/// // #[ink(storage)]
/// // pub struct MyContract {
/// //     // storage fields
/// // }
///
/// // We can represent it using `syn::Item`:
/// let items: Vec<Item> = vec![parse_quote! {
///     #[ink(storage)]
///     pub struct MyContract {
///         // storage fields
///     }
/// }];
///
/// let storage_struct_name = extract_storage_struct_name(&items);
/// assert_eq!(storage_struct_name, "MyContract");
/// ```
///
pub fn extract_storage_struct_name(items: &[syn::Item]) -> String {
    let contract_storage_struct = items
        .iter()
        .find(|item| {
            if let Item::Struct(structure) = item {
                let ink_attr_maybe = structure
                    .attrs
                    .iter()
                    .find(|&attr| is_attr(&[attr.clone()], "ink"))
                    .cloned();

                if let Some(ink_attr) = ink_attr_maybe {
                    if let Ok(path) = ink_attr.parse_args::<Path>() {
                        return path.to_token_stream().to_string() == "storage";
                    }
                }
                false
            } else {
                false
            }
        })
        .expect("Contract storage struct not found!");
    match contract_storage_struct {
        Item::Struct(structure) => structure.ident.to_string(),
        _ => unreachable!("Only Item::Struct allowed here"),
    }
}
