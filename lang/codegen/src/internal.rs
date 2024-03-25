// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

extern crate proc_macro;

use quote::ToTokens;
use std::collections::HashSet;
use syn::{
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    FnArg,
};

pub(crate) struct MetaList {
    pub _path: syn::Path,
    pub _paren_token: syn::token::Paren,
    pub _nested: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
}
#[derive(Debug)]
pub struct InputsDiff {
    pub added: HashSet<String>,
    pub removed: HashSet<String>,
}

pub fn inputs_diff(
    inputs_a: Punctuated<FnArg, Comma>,
    inputs_b: Punctuated<FnArg, Comma>,
) -> InputsDiff {
    let set_a: HashSet<_> = inputs_a
        .into_iter()
        .map(|arg| arg.into_token_stream().to_string())
        .collect();
    let set_b: HashSet<_> = inputs_b
        .into_iter()
        .map(|arg| arg.into_token_stream().to_string())
        .collect();

    let added = &set_b - &set_a;
    let removed = &set_a - &set_b;

    InputsDiff {
        added: added.into_iter().collect(),
        removed: removed.into_iter().collect(),
    }
}

pub fn format_arg_string(arg: &str) -> String {
    let mut formatted = String::new();
    let mut chars = arg.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '&' => {
                if chars.peek() == Some(&' ') {
                    formatted.push('&');
                    chars.next(); // Remove space after '&'
                }
            }
            _ => {
                if chars.peek() == Some(&':') {
                    formatted.push(':');
                    chars.next();
                } else if chars.peek() == Some(&'>') {
                    formatted.push('>');
                    chars.next();
                    chars.next();
                } else if chars.peek() == Some(&'<') {
                    formatted.push('<');
                    chars.next();
                    chars.next();
                } else if chars.peek() == Some(&',') {
                    formatted.push(',');
                    chars.next();
                } else {
                    formatted.push(ch);
                }
            }
        }
    }

    formatted
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
                    break;
                }
                let punct = input.parse()?;
                segments.push_punct(punct);
            }
            if segments.is_empty() {
                return Err(input.error("expected path"));
            } else if segments.trailing_punct() {
                return Err(input.error("expected path segment"));
            }
            segments
        },
    })
}

fn parse_meta_list_after_path(
    path: syn::Path,
    input: ParseStream,
) -> syn::Result<MetaList> {
    let content;
    Ok(MetaList {
        _path: path,
        _paren_token: parenthesized!(content in input),
        _nested: content.parse_terminated(syn::Expr::parse)?,
    })
}

fn parse_meta_after_path(
    path: syn::Path,
    input: ParseStream,
) -> syn::Result<NestedMeta> {
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
                break;
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

pub(crate) const INK_PREFIX: &str = "ink=";

#[inline]
pub(crate) fn skip() -> bool {
    !std::env::args().any(|arg| arg.contains(INK_PREFIX))
}
