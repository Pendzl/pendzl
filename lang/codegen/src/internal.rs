// Copyright (c) 2012-2022 Supercolony. All Rights Reserved.
// Copyright (c) 2023 Brushfam. All Rights Reserved.
// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

extern crate proc_macro;

use quote::ToTokens;
use std::collections::HashSet;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    FnArg,
};

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

pub(crate) struct AttributeArgs(Vec<syn::Path>);

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
    type Target = Vec<syn::Path>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AttributeArgs {
    fn deref_mut(&mut self) -> &mut Vec<syn::Path> {
        &mut self.0
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
