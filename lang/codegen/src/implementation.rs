// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

use crate::{implementations::*, internal, internal::*};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{Item, Path};

pub fn generate(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    if internal::skip() {
        return quote! {};
    }
    let input: TokenStream = ink_module;

    // map attribute args to default contract names
    let args = syn::parse2::<AttributeArgs>(attrs)
        .expect("No default contracts to implement provided")
        .iter()
        .map(|arg| match arg {
            NestedMeta::Path(method) => {
                method.to_token_stream().to_string().replace(' ', "")
            }
            _ => panic!(
                "Expected names of pendzl traits to implement in the contract!"
            ),
        })
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

    for to_default_implement in &args {
        match to_default_implement.as_str() {
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
            _ => panic!("pendzl::implementation({to_default_implement}) not implemented!"),
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

//TODO verify
fn cleanup_imports(imports: &mut HashMap<&str, syn::ItemUse>) {
    // we will remove unnecessary imports
    let psp22_default_impls =
        vec!["PSP22Mintable", "PSP22Burnable", "PSP22Metadata"];
    check_and_remove_import("PSP22", psp22_default_impls, imports);

    let psp34_default_impls =
        vec!["PSP34Mintable", "PSP34Burnable", "PSP34Metadata"];
    check_and_remove_import("PSP34", psp34_default_impls, imports);
}

fn check_and_remove_import(
    name_to_check: &str,
    to_check: Vec<&str>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    if to_check.iter().any(|name| imports.contains_key(name)) {
        imports.remove(name_to_check);
    }
}

// this method consumes override annotated methods and returns then mapped to code and the mod without them
// we will later override the methods
fn consume_overriders(
    items: Vec<syn::Item>,
) -> (OverridenFnMap, Vec<syn::Item>) {
    let mut map = HashMap::new();
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Fn(item_fn) = &mut item {
            if is_attr(&item_fn.attrs, "overrider") {
                let attr_name = "overrider";
                let fn_name = item_fn.sig.ident.to_string();
                let code = item_fn.block.clone();
                let mut attributes = item_fn.attrs.clone();
                let inputs = item_fn.sig.inputs.clone();
                // remove the overrider attribute
                let to_remove_idx = attributes
                    .iter()
                    .position(|attr| is_attr(&[attr.clone()], attr_name))
                    .expect("No {attr_name} attribute found!");
                let overrider_attribute = attributes.remove(to_remove_idx);

                let trait_name = overrider_attribute
                    .parse_args::<Path>()
                    .expect("Expected overriden trait identifier")
                    .to_token_stream()
                    .to_string();

                let mut vec = map.get(&trait_name).unwrap_or(&vec![]).clone();
                vec.push((fn_name, (code, attributes, inputs)));
                map.insert(trait_name, vec.to_vec());
            } else {
                result.push(item);
            }
        } else {
            result.push(item);
        }
    });

    (map, result)
}

pub fn extract_storage_struct_name(items: &[syn::Item]) -> String {
    let contract_storage_struct = items
        .iter()
        .find(|item| {
            if let Item::Struct(structure) = item {
                let ink_attr_maybe = structure
                    .attrs
                    .iter()
                    .cloned()
                    .find(|attr| is_attr(&[attr.clone()], "ink"));

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
