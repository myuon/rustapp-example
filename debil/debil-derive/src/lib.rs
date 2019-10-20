extern crate proc_macro;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result};

struct TableAttr {
    table_name: String,
    sql_type: proc_macro2::TokenStream,
}

struct FieldAttr {
    sql_type: String,
    size: String,
    column_name: String,
    column_type: String,
}

struct AttrInput {
    paren_token: syn::token::Paren,
    attrs: syn::punctuated::Punctuated<KeyValue, syn::Token![,]>,
}

impl Parse for AttrInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(AttrInput {
            paren_token: syn::parenthesized!(content in input),
            attrs: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

impl AttrInput {
    fn to_table_attr(self, table_name: String) -> TableAttr {
        let mut table = TableAttr {
            table_name: table_name,
            sql_type: quote! { Vec<u8> },
        };

        for attr in self.attrs.into_iter() {
            match format!("{}", attr.key).as_str() {
                "table_name" => table.table_name = attr.value.value(),
                "sql_type" => {
                    let sql_type = quote::format_ident!("{}", &attr.value.value());
                    table.sql_type = quote! { #sql_type };
                }
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

fn get_fields_from_datastruct(data: syn::Data) -> Vec<proc_macro2::Ident> {
    let mut result = Vec::new();

    match data {
        syn::Data::Struct(st) => match st.fields {
            syn::Fields::Named(fields) => {
                for name in fields.named.iter() {
                    result.push(name.ident.as_ref().unwrap().clone());
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    result
}

#[proc_macro_derive(Record, attributes(sql))]
pub fn derive_record(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let attr_stream = input.attrs[0].tokens.clone();
    let table_attr = syn::parse2::<AttrInput>(attr_stream)
        .unwrap()
        .to_table_attr(format!("{}", ident));
    let table_name = table_attr.table_name;

    let field_names = get_fields_from_datastruct(input.data);
    let push_field_names = field_names
        .iter()
        .map(|ident| quote! { result.push((stringify!(#ident).to_string(), SQLValue::serialize(self.#ident))); })
        .collect::<Vec<_>>();

    let sql_type = table_attr.sql_type;

    let expanded = quote! {
        impl SQLTable for #ident {
            type Type = #sql_type;

            fn table_name(&self) -> &'static str {
                #table_name
            }

            fn map_to_sql<V: SQLValue<Self::Type>>(self) -> Vec<(String, V)> {
                let result = Vec::new();
                #( #push_field_names )*

                result
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
