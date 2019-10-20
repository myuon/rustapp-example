extern crate proc_macro;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result};

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

impl TableAttrInput {
    fn to_table_attr(self, table_name: String) -> TableAttr {
        let mut table = TableAttr {
            table_name: table_name,
        };

        for attr in self.attrs.into_iter() {
            match format!("{}", attr.key).as_str() {
                "table_name" => table.table_name = attr.value.value(),
                d => panic!("unsupported attribute: {}", d),
            }
        }

        table
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

#[proc_macro_derive(Record, attributes(sql))]
pub fn derive_record(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let attr_stream = input.attrs[0].tokens.clone();
    let table_attr = syn::parse2::<TableAttrInput>(attr_stream)
        .unwrap()
        .to_table_attr(format!("{}", ident));
    let table_name = table_attr.table_name;

    let expanded = quote! {
        impl SQLTable for #ident {
            fn table_name(&self) -> &'static str {
                #table_name
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
