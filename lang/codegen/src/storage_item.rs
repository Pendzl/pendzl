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

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, Field, Fields};

#[inline]
pub(crate) fn is_attr(attrs: &[syn::Attribute], ident: &str) -> bool {
    attrs.iter().any(|attr| {
        attr.path
            .segments
            .last()
            .expect("No segments in path")
            .ident
            == ident
    })
}

fn wrap_fields_to_be_upgradeable_safely(
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

fn generate_struct(s: &synstructure::Structure, struct_item: DataStruct) -> TokenStream {
    let struct_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let types = s.ast().generics.clone();
    let attrs = s.ast().attrs.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();

    let (fields, storage_keys) = wrap_fields_to_be_upgradeable_safely(
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

fn generate_enum(s: &synstructure::Structure, enum_item: DataEnum) -> TokenStream {
    let enum_ident = s.ast().ident.clone();
    let vis = s.ast().vis.clone();
    let attrs = s.ast().attrs.clone();
    let types = s.ast().generics.clone();
    let (_, _, where_closure) = s.ast().generics.split_for_impl();
    let mut all_storage_keys: Vec<Option<TokenStream>> = vec![];

    //
    // enum ExampleEnum {
    //     Unit,
    //     ExUnNamed(bool) = 123,
    //     ExNamed{a: bool},
    // }

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
        let (fields, storage_keys) = wrap_fields_to_be_upgradeable_safely(
            format!("{}_{}", enum_ident, variant_ident).as_str(),
            variant.fields.clone(),
        );

        let fields = match variant.fields {
            Fields::Named(_) => quote! { { #(#fields),* } },
            Fields::Unnamed(_) => quote! { ( #(#fields),* ) },
            Fields::Unit => quote! {},
        };

        // push keys to array - defer generating to the outside of enum
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

pub fn storage_item(_attrs: TokenStream, s: synstructure::Structure) -> TokenStream {
    let item = match s.ast().data.clone() {
        Data::Struct(struct_item) => generate_struct(&s, struct_item),
        Data::Enum(enum_item) => generate_enum(&s, enum_item),
        Data::Union(_) => panic!("{} - pendzl storage_item cannot wrap Union", s.ast().ident.clone())
    };

    quote! {
        #[::ink::storage_item]
        #item
    }
}
