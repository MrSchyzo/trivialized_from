mod metadata;

extern crate proc_macro;
extern crate proc_macro_error;

use crate::metadata::attributes::render_field_expression;
use metadata::attributes::from::FromMetadata;
use metadata::ParseError;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_error, proc_macro_error};
use quote::__private::Ident;
use quote::{quote, ToTokens};
use syn;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Type};

#[proc_macro_error]
#[proc_macro_derive(TrivializationReady, attributes(Into, From, Transform, MacroTransform))]
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

//TODO: implement Into and Transform
fn handle_enum_case(
    types_to_convert: Vec<Type>,
    derive_name: &Ident,
    enumeration: DataEnum,
) -> TokenStream {
    let impl_blocks = types_to_convert.iter().map(|ty| {
        let variant_mappings = enumeration.variants.iter().map(|v| {
            let variant_name = &v.ident;
            //If variant Transform, just other::variant(_) => transform(e)
            //If variant Into, other::variant(f0, ..., fn) => {= all fields have #[Into]}
            //If field Transform, (f0, transform(f0))
            //If field Into, and f0 = 
            /*
            let (intos, errors): (Vec<_>, Vec<_>) = v.attrs.iter().map(IntoMetadata::maybe_from).partition(Result::is_ok);
            
            if errors.len() > 0 {
                return Err(errors.into_iter().filter_map(Result::err).collect())
            }
            */
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
    let (mappings, errors): (Vec<_>, Vec<_>) = structure
        .fields
        .iter()
        .map(render_field_expression)
        .partition(Result::is_ok);

    if errors.len() > 0 {
        errors
            .into_iter()
            .filter_map(Result::err)
            .flat_map(|errs| errs)
            .for_each(|e| {
                emit_error!(e.span, e.message);
            });
        TokenStream::new()
    } else {
        render_struct_implementors(
            types_to_convert,
            derive_name,
            mappings.into_iter().filter_map(Result::ok).collect(),
        )
    }
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
    let (results, errors): (Vec<_>, Vec<_>) = derive_input
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
        .filter_map(std::convert::identity)
        .reduce(FromMetadata::merge)
        .unwrap_or_default()
        .types()
}
