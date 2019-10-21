extern crate proc_macro;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result};

struct TableAttr {
    table_name: String,
    sql_type: proc_macro2::TokenStream,
}

struct FieldAttr {
    size: proc_macro2::TokenStream,
    not_null: proc_macro2::TokenStream,
    unique: proc_macro2::TokenStream,
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
                "table_name" => {
                    table.table_name = syn::parse2::<Universe>(attr.value)
                        .unwrap()
                        .as_str()
                        .unwrap()
                }
                "sql_type" => {
                    let sql_type = quote::format_ident!(
                        "{}",
                        &syn::parse2::<Universe>(attr.value)
                            .unwrap()
                            .as_str()
                            .unwrap()
                    );
                    table.sql_type = quote! { #sql_type };
                }
                d => panic!("unsupported attribute: {}", d),
            }
        }

        table
    }

    fn to_field_attr(self) -> FieldAttr {
        let mut field = FieldAttr {
            size: quote! { None },
            not_null: quote! { false },
            unique: quote! { false },
        };

        for attr in self.attrs.into_iter() {
            match format!("{}", attr.key).as_str() {
                "size" => field.size = attr.value,
                "not_null" => field.not_null = attr.value,
                "unique" => field.unique = attr.value,
                d => panic!("unsupported attribute: {}", d),
            }
        }

        field
    }
}

enum Universe {
    VStr(String),
    VI32(i32),
    VBool(bool),
}

impl Parse for Universe {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            input
                .parse::<syn::LitStr>()
                .map(|v| Universe::VStr(v.value()))
        } else if lookahead.peek(syn::LitInt) {
            input
                .parse::<syn::LitInt>()
                .map(|v| Universe::VI32(v.base10_parse::<i32>().unwrap()))
        } else if lookahead.peek(syn::LitBool) {
            input
                .parse::<syn::LitBool>()
                .map(|v| Universe::VBool(v.value))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Universe {
    fn as_str(self) -> Option<String> {
        use Universe::*;
        match self {
            VStr(s) => Some(s),
            _ => None,
        }
    }

    fn as_i32(self) -> Option<i32> {
        use Universe::*;
        match self {
            VI32(i) => Some(i),
            _ => None,
        }
    }

    fn as_bool(self) -> Option<bool> {
        use Universe::*;
        match self {
            VBool(b) => Some(b),
            _ => None,
        }
    }
}

struct KeyValue {
    key: proc_macro2::Ident,
    punct: syn::Token![=],
    value: proc_macro2::TokenStream,
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

fn get_fields_from_datastruct(data: syn::Data) -> Vec<(proc_macro2::Ident, syn::Type, FieldAttr)> {
    let mut result = Vec::new();

    match data {
        syn::Data::Struct(st) => match st.fields {
            syn::Fields::Named(fields) => {
                for name in fields.named.iter() {
                    result.push((
                        name.ident.as_ref().unwrap().clone(),
                        name.ty.clone(),
                        if name.attrs.len() == 0 {
                            FieldAttr {
                                size: quote! { None },
                                not_null: quote! { false },
                                unique: quote! { false },
                            }
                        } else {
                            // TODO: Only first FieldAttr will be effective
                            syn::parse2::<AttrInput>(name.attrs[0].tokens.clone())
                                .unwrap()
                                .to_field_attr()
                        },
                    ));
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    result
}

#[proc_macro_derive(Table, attributes(sql))]
pub fn derive_record(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let attr_stream = input.attrs[0].tokens.clone();
    let table_attr = syn::parse2::<AttrInput>(attr_stream)
        .unwrap()
        .to_table_attr(format!("{}", ident));
    let table_name = table_attr.table_name;

    let field_struct = get_fields_from_datastruct(input.data);
    let push_field_names = field_struct
        .iter()
        .map(|(ident, _, _)| quote! { result.push((stringify!(#ident).to_string(), SQLValue::serialize(self.#ident))); })
        .collect::<Vec<_>>();
    let push_column_schema = field_struct
        .iter()
        .map(move |(ident, ty, attr)| {
            let size = &attr.size;
            let unique = &attr.unique;
            let not_null = &attr.not_null;

            quote! {
                result.push((stringify!(#ident).to_string(), SQLValue::column_type(std::marker::PhantomData::<#ty>), FieldAttribute {
                    size: #size,
                    unique: #unique,
                    not_null: #not_null,
                }));
            }
        })
        .collect::<Vec<_>>();

    let sql_type = table_attr.sql_type;

    let expanded = quote! {
        impl SQLTable for #ident {
            type Type = #sql_type;

            fn table_name(&self) -> &'static str {
                #table_name
            }

            fn schema_of(&self) -> Vec<(String, String, FieldAttribute)> {
                let result = Vec::new();
                #( #push_column_schema )*

                result
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
