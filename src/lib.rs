mod metadata;

extern crate proc_macro;
extern crate proc_macro_error;

use crate::metadata::attributes::{
    compose_transformations, parse_transformations, render_expression, render_field_expression,
    render_transform_expression, FieldTransformation,
};
use metadata::attributes::from::FromMetadata;
use metadata::ParseError;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_error, proc_macro_error};
use quote::__private::Ident;
use quote::{quote, ToTokens};
use syn;
use syn::spanned::Spanned;
use syn::{AttrStyle, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Type};

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

//TODO: refactor this smoking pile of junk and add more error handling
fn handle_enum_case(
    types_to_convert: Vec<Type>,
    derive_name: &Ident,
    enumeration: DataEnum,
) -> TokenStream {
    let impl_blocks = types_to_convert.iter().map(|ty| {
        let variant_mappings = enumeration.variants.iter().map(|v| {
            let variant_name = &v.ident;

            let (transformations, errors): (Vec<_>, Vec<_>) = parse_transformations(&v.attrs).into_iter().partition(Result::is_ok);
            let transformations = transformations.into_iter().filter_map(Result::ok).filter(|r| !(matches!(r, FieldTransformation::Nothing))).collect::<Vec<_>>();
            let errors = errors.into_iter().filter_map(Result::err).collect::<Vec<_>>();

            if errors.len() > 0 {
                errors.into_iter().for_each(|e: ParseError| emit_error!(e.span, e.message));
                return quote!()
            }

            let is_there_an_into = transformations.iter().any(|r| matches!(r, FieldTransformation::Into(_)));
            let is_invalid = is_there_an_into && transformations.len() > 1;

            if is_invalid {
                emit_error!(v.clone().span(), "Variant must have an #[Into] only, or a combination of multiple #[Transform] and #[MacroTransform], or without attributes".to_owned());
                return quote!()
            }

            if !is_there_an_into && !is_invalid && transformations.len() > 0 {
                let mapping = compose_transformations(quote!(other), &transformations, &ty.to_token_stream().to_string());
                return match &v.fields {
                    Fields::Named(_) => quote!(#ty::#variant_name{ .. } => (#mapping)),
                    Fields::Unnamed(_) => quote!(#ty::#variant_name(_) => (#mapping)),
                    Fields::Unit => quote!(#ty::#variant_name => (#mapping)),
                };
            }

            let new_fields = &v.fields.iter().enumerate().map(|(i, f): (usize, &Field)| Field {
                ident: f.ident.clone().or_else(|| syn::parse_str::<syn::Ident>(&(format!("field_number_{}", i))).ok()),
                attrs: vec![Attribute{
                    pound_token: syn::token::Pound::default(),
                    style: AttrStyle::Outer,
                    bracket_token: Default::default(),
                    path: syn::parse_str::<syn::Path>("Into").unwrap(),
                    tokens: Default::default()
                }].into_iter()
                    .filter(|_| is_there_an_into)
                    .chain(f.attrs.clone().into_iter())
                    .collect(),
                ..(f.clone())
            }).collect::<Vec<_>>();

            let new_idents = new_fields.iter().map(|f| f.ident.as_ref().unwrap());

            let (expressions, errors): (Vec<_>, Vec<_>) = (match &v.fields {
                Fields::Named(_) => new_fields.iter().map(|f: &Field| render_expression(f)).collect::<Vec<_>>(),
                Fields::Unnamed(_) => new_fields.iter().map(|f: &Field| render_transform_expression(f.ident.as_ref().unwrap(), f)).collect::<Vec<_>>(),
                Fields::Unit => Vec::new(),
            }).into_iter().partition(Result::is_ok);
            let expressions = expressions.into_iter().filter_map(Result::ok).collect::<Vec<_>>();
            let errors = errors.into_iter().filter_map(Result::err).collect::<Vec<_>>();

            if errors.len() > 0 {
                errors.into_iter().flat_map(|f| f).for_each(|e: ParseError| emit_error!(e.span, e.message));
                return quote!()
            }

            match &v.fields {
                Fields::Named(_) => quote!{#ty::#variant_name{#(#new_idents,)*} => Self::#variant_name{#(#expressions,)*}},
                Fields::Unnamed(_) => quote!{#ty::#variant_name(#(#new_idents,)*) => Self::#variant_name(#(#expressions,)*)},
                _ => quote!{#ty::#variant_name => Self::#variant_name},
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
