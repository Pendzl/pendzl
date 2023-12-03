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

extern crate proc_macro;

use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{
        Parse,
        ParseStream,
    },
};

pub(crate) struct MetaList {
    pub _path: syn::Path,
    pub _paren_token: syn::token::Paren,
    pub _nested: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}

// Like Path::parse_mod_style but accepts keywords in the path.
fn parse_meta_path(input: ParseStream) -> syn::Result<syn::Path> {
    Ok(syn::Path {
        leading_colon: input.parse()?,
        segments: {
            let mut segments = syn::punctuated::Punctuated::new();
            while input.peek(syn::Ident::peek_any) {
                let ident = syn::Ident::parse_any(input)?;
                segments.push_value(syn::PathSegment::from(ident));
                if !input.peek(syn::Token![::]) {
                    break
                }
                let punct = input.parse()?;
                segments.push_punct(punct);
            }
            if segments.is_empty() {
                return Err(input.error("expected path"))
            } else if segments.trailing_punct() {
                return Err(input.error("expected path segment"))
            }
            segments
        },
    })
}

fn parse_meta_list_after_path(path: syn::Path, input: ParseStream) -> syn::Result<MetaList> {
    let content;
    Ok(MetaList {
        _path: path,
        _paren_token: parenthesized!(content in input),
        _nested: content.parse_terminated(syn::Expr::parse)?,
    })
}

fn parse_meta_after_path(path: syn::Path, input: ParseStream) -> syn::Result<NestedMeta> {
    if input.peek(syn::token::Paren) {
        parse_meta_list_after_path(path, input).map(NestedMeta::List)
    } else {
        Ok(NestedMeta::Path(path))
    }
}

impl Parse for MetaList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.call(parse_meta_path)?;
        parse_meta_list_after_path(path, input)
    }
}

pub(crate) enum NestedMeta {
    Path(syn::Path),
    List(MetaList),
}

impl Parse for NestedMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.call(parse_meta_path)?;
        parse_meta_after_path(path, input)
    }
}

pub(crate) struct AttributeArgs(Vec<NestedMeta>);

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();
        while input.peek(syn::Ident::peek_any) {
            attrs.push(input.parse()?);
            if input.is_empty() {
                break
            }
            let _: syn::token::Comma = input.parse()?;
        }
        Ok(AttributeArgs(attrs))
    }
}

impl std::ops::Deref for AttributeArgs {
    type Target = Vec<NestedMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AttributeArgs {
    fn deref_mut(&mut self) -> &mut Vec<NestedMeta> {
        &mut self.0
    }
}

pub(crate) struct Attributes(Vec<syn::Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(syn::Attribute::parse_outer(input)?))
    }
}
// impl Attributes {
//     pub(crate) fn attr(&self) -> &Vec<syn::Attribute> {
//         &self.0
//     }
// }

#[inline]
pub(crate) fn is_attr(attrs: &[syn::Attribute], ident: &str) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.segments.last().expect("No segments in path").ident == ident)
}

// #[inline]
// #[allow(dead_code)]
// pub(crate) fn get_attr(attrs: &[syn::Attribute], ident: &str) -> Option<syn::Attribute> {
//     for attr in attrs.iter() {
//         if is_attr(&[attr.clone()], ident) {
//             return Some(attr.clone())
//         }
//     }
//     None
// }

// #[inline]
// pub(crate) fn remove_attr(attrs: &[syn::Attribute], ident: &str) -> Vec<syn::Attribute> {
//     attrs
//         .iter()
//         .cloned()
//         .filter_map(|attr| {
//             if is_attr(&[attr.clone()], ident) {
//                 None
//             } else {
//                 Some(attr)
//             }
//         })
//         .collect()
// }

// #[inline]
// pub(crate) fn extract_attr(attrs: &mut Vec<syn::Attribute>, ident: &str) -> Vec<syn::Attribute> {
//     let extracted = attrs
//         .clone()
//         .into_iter()
//         .filter(|attr| is_attr(&[attr.clone()], ident))
//         .collect();
//     attrs.retain(|attr| !is_attr(&[attr.clone()], ident));
//     extracted
// }

// #[inline]
// pub(crate) fn new_attribute(attr_stream: TokenStream) -> syn::Attribute {
//     syn::parse2::<Attributes>(attr_stream).unwrap().attr()[0].clone()
// }

pub(crate) const INK_PREFIX: &str = "ink=";

#[inline]
pub(crate) fn skip() -> bool {
    !std::env::args().any(|arg| arg.contains(INK_PREFIX))
}
