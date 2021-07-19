extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn;
use syn::{Meta, NestedMeta, Path, Data, Type, DeriveInput};
use std::collections::HashSet;
use quote::__private::Ident;
use syn::__private::TokenStream2;

#[proc_macro_derive(TrivializationReady, attributes(Into, From, Transform))]
pub fn derive_trivialization_ready(structure: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(structure).unwrap();

    let types: Vec<syn::Type> = types_to_be_converted_from(&derive_input);
    let struct_name = &derive_input.ident;
    let structure = if let Data::Struct(s) = derive_input.data {
        s
    } else {
        panic!("Only struct is supported!")
    };

    let fields = if let syn::Fields::Named(ref named) = structure.fields {
        named
    } else {
        unimplemented!("Need named fields in struct!")
    };

    let field_mappings: Vec<TokenStream2> = fields.named.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let type_path = if let syn::Type::Path(syn::TypePath {path: p, ..}) = &f.ty {
            p.clone()
        } else {
            unimplemented!("Need a TypePath!")
        };
        let attrs = &f.attrs;
        if attrs.iter().any(|a| as_name(&a.path).eq("Transform")) {
            let att = attrs.iter().find(|a| as_name(&a.path).eq("Transform")).unwrap();
            let foo = if let Ok(Meta::List(l)) = att.parse_meta() {
                if let NestedMeta::Meta(Meta::Path(ref p)) = l.nested.first().unwrap() {
                    as_name(p)
                } else {
                    panic!("Unexpected Meta in attribute Transform")
                }
            } else {
                panic!("Unexpected Meta type for attribute Transform")
            };
            let call = syn::parse_str::<syn::ExprPath>(&foo).unwrap();
            quote! {#name: #call(other.#name)}
        } else if attrs.iter().any(|a| as_name(&a.path).eq("Into")) {
            if as_name(&type_path).ends_with("Option") {
                quote!{#name: other.#name.map(Into::into)}
            } else if as_name(&type_path).ends_with("Vec") {
                quote!{#name: other.#name.into_iter().map(Into::into).collect()}
            } else {
                quote!{#name: other.#name.into()}
            }
        } else {
            quote!{#name: other.#name}
        }
    }).collect();
    render_struct_implementors(types, struct_name, field_mappings)
}

fn render_struct_implementors<Field: ToTokens>(types: Vec<Type>, struct_name: &Ident, field_mappings: Vec<Field>) -> TokenStream {
    let impl_blocks = types.iter().map(|ty| {
        quote! {
            impl From<#ty> for #struct_name {
                fn from(other: #ty) -> Self {
                    Self {
                        #(#field_mappings,)*
                    }
                }
            }
        }
    });

    (quote! {#(#impl_blocks)*}).into()
}

//TODO: better error handling, now it's full of unwrap and panics... ew
fn types_to_be_converted_from(derive_input: &DeriveInput) -> Vec<Type> {
    derive_input.attrs.iter()
        .filter(|a| as_name(&a.path).eq("From"))
        .map(|a| a.parse_meta())
        .map(|meta| match meta {
            Ok(Meta::List(l)) => l.clone().nested.iter().map(|nested| match nested {
                NestedMeta::Meta(Meta::Path(ref p)) => as_name(&p),
                _ => panic!("Unrecognized NestedMeta"),
            }).collect::<Vec<String>>(),
            _ => panic!("Unrecognized Meta"),
        })
        .flatten()
        .collect::<HashSet<String>>()
        .iter()
        .map(|t| syn::parse_str::<syn::Type>(t).unwrap())
        .collect()
}

fn as_name(p: &Path) -> String {
    p.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("::")
}