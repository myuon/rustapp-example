extern crate proc_macro;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result};

struct TokenWrapper(proc_macro2::TokenTree);

impl TokenWrapper {
    fn as_group(&self) -> Option<&proc_macro2::Group> {
        use proc_macro2::TokenTree::*;

        match &self.0 {
            Group(v) => Some(v),
            _ => None,
        }
    }

    fn as_ident(&self) -> Option<&proc_macro2::Ident> {
        use proc_macro2::TokenTree::*;

        match &self.0 {
            Ident(v) => Some(v),
            _ => None,
        }
    }

    fn as_punct(&self, ch: char) -> Option<&proc_macro2::Punct> {
        use proc_macro2::TokenTree::*;

        match &self.0 {
            Punct(v) if v.as_char() == ch => Some(v),
            _ => None,
        }
    }

    fn as_lit(&self) -> Option<&proc_macro2::Literal> {
        use proc_macro2::TokenTree::*;

        match &self.0 {
            Literal(v) => Some(v),
            _ => None,
        }
    }
}

struct TableAttr {
    table_name: String,
}

struct TableAttrInput {
    paren_token: syn::token::Paren,
    attrs: syn::punctuated::Punctuated<KeyValue, syn::Token![,]>,
}

impl Parse for TableAttrInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(TableAttrInput {
            paren_token: syn::parenthesized!(content in input),
            attrs: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

struct KeyValue {
    key: proc_macro2::Ident,
    punct: syn::Token![=],
    value: syn::LitStr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            punct: input.parse()?,
            value: input.parse()?,
        })
    }
}

/*
fn parse_tokens_into_pairs(stream: proc_macro2::TokenStream) -> Vec<(String, String)> {
    let mut iter = stream.into_iter();
    let mut pairs = Vec::new();

    let mut parse_key_value = |iter: &mut proc_macro2::token_stream::IntoIter| {
        let key_token = TokenWrapper(iter.next()?);
        let key = key_token.as_ident()?;

        TokenWrapper(iter.next()?).as_punct('=')?;

        let value_token = TokenWrapper(iter.next()?);
        let value = value_token.as_lit()?;

        pairs.push((format!("{}", key), value));

        Some(())
    };

    loop {
        let r = parse_key_value(&mut iter);
        if r.is_none() {
            break;
        }
    }

    pairs
}
*/

#[proc_macro_derive(Record, attributes(sql))]
pub fn derive_record(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let attr_stream = input.attrs[0].tokens.clone();
    let pairs = syn::parse2::<TableAttrInput>(attr_stream).unwrap();
    let table_name = pairs.attrs.first().unwrap().value.value();

    let expanded = quote! {
        impl SQLTable for #ident {
            fn table_name(&self) -> &'static str {
                #table_name
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
