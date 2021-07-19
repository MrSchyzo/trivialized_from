extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Meta, NestedMeta, Path, Data};

#[proc_macro_derive(TrivializationReady, attributes(Into, From))]
pub fn derive_trivialization_ready(structure: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(structure).unwrap();
    let types: Vec<syn::Type> = derive_input.attrs.iter()
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
        .map(|s| syn::parse_str::<syn::Type>(s.as_str()).unwrap())
        .collect();

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

    let converted_fields: Vec<_> = fields.named.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let type_path = if let syn::Type::Path(syn::TypePath {path: p, ..}) = &f.ty {
            p.clone()
        } else {
            unimplemented!("Need a TypePath!")
        };
        let attrs = &f.attrs;
        if attrs.iter().any(|a| as_name(&a.path).eq("Into")) {
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
    let impl_blocks = types.iter().map(|ty| {
        quote! {
            impl From<#ty> for #struct_name {
                fn from(other: #ty) -> #struct_name {
                    #struct_name {
                        #(#converted_fields,)*
                    }
                }
            }

        }
    });

    (quote! {#(#impl_blocks)*}).into()
}

fn as_name(p: &Path) -> String {
    p.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("::")
}