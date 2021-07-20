mod metadata;

extern crate proc_macro;
extern crate proc_macro_error;

use metadata::{as_name, FromMetadata, ParseError};
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_error, proc_macro_error};
use quote::__private::Ident;
use quote::{quote, ToTokens};
use syn;
use syn::__private::TokenStream2;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Meta, NestedMeta, Type};

#[proc_macro_error]
#[proc_macro_derive(TrivializationReady, attributes(Into, From, Transform))]
pub fn derive_trivialization_ready(structure: TokenStream) -> TokenStream {
    let derive_input: syn::DeriveInput = match syn::parse::<syn::DeriveInput>(structure) {
        Ok(input) => input,
        Err(e) => abort_call_site!(format!(
            "Unable to parse data structure under #[derive(...)]: {}",
            e
        )),
    };

    let types_to_convert: Vec<syn::Type> = match types_to_convert(&derive_input) {
        Ok(types) => types,
        Err(errors) => {
            errors.iter().for_each(|e| emit_error!(e.span, e.message));
            return TokenStream::new();
        }
    };

    match derive_input.data {
        Data::Struct(s) => handle_struct_case(types_to_convert, &derive_input.ident, s),
        Data::Enum(e) => handle_enum_case(types_to_convert, &derive_input.ident, e),
        Data::Union(_) => panic!("TrivializationReady works only for `struct`s and `enum`s"),
    }
}

fn handle_enum_case(
    types_to_convert: Vec<Type>,
    derive_name: &Ident,
    enumeration: DataEnum,
) -> TokenStream {
    let impl_blocks = types_to_convert.iter().map(|ty| {
        let variant_mappings = enumeration.variants.iter().map(|v| {
            let variant_name = &v.ident;
            let mut index = 0;
            let field_mappings = v.fields.iter().map(|_f| {
                let id = syn::parse_str::<syn::Ident>(format!("field{}", index).as_str()).unwrap();
                index+=1;
                quote!{#id}
            }).collect::<Vec<_>>();
            if field_mappings.len() > 0 {
                quote!{#ty::#variant_name(#(#field_mappings,)*) => Self::#variant_name(#(#field_mappings,)*)}
            } else {
                quote!{#ty::#variant_name => Self::#variant_name}
            }
        }).collect::<Vec<_>>();

        quote! {
            impl From<#ty> for #derive_name {
                fn from(other: #ty) -> Self {
                    match other {
                        #(#variant_mappings,)*
                    }
                }
            }
        }
    });

    (quote! {#(#impl_blocks)*}).into()
}

fn handle_struct_case(
    types_to_convert: Vec<Type>,
    derive_name: &Ident,
    structure: DataStruct,
) -> TokenStream {
    let field_mappings: Vec<TokenStream2> = structure
        .fields
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let type_path = if let syn::Type::Path(syn::TypePath { path: p, .. }) = &f.ty {
                p.clone()
            } else {
                unimplemented!("Need a TypePath!")
            };
            let attrs = &f.attrs;
            if attrs.iter().any(|a| as_name(&a.path).eq("Transform")) {
                let att = attrs
                    .iter()
                    .find(|a| as_name(&a.path).eq("Transform"))
                    .unwrap();
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
                    quote! {#name: other.#name.map(Into::into)}
                } else if as_name(&type_path).ends_with("Vec") {
                    quote! {#name: other.#name.into_iter().map(Into::into).collect()}
                } else {
                    quote! {#name: other.#name.into()}
                }
            } else {
                quote! {#name: other.#name}
            }
        })
        .collect();

    render_struct_implementors(types_to_convert, derive_name, field_mappings)
}

fn render_struct_implementors<Field: ToTokens>(
    types: Vec<Type>,
    struct_name: &Ident,
    field_mappings: Vec<Field>,
) -> TokenStream {
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

fn types_to_convert(derive_input: &DeriveInput) -> Result<Vec<Type>, Vec<ParseError>> {
    let (results, errors): (
        Vec<Result<Option<FromMetadata>, Vec<ParseError>>>,
        Vec<Result<Option<FromMetadata>, Vec<ParseError>>>,
    ) = derive_input
        .attrs
        .iter()
        .map(FromMetadata::maybe_from)
        .partition(Result::is_ok);

    if errors.len() > 0 {
        return Err(errors.into_iter().flat_map(Result::err).flatten().collect());
    }

    results
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|x| x)
        .reduce(|a, b| a.merge(b))
        .unwrap_or_default()
        .types()
}
