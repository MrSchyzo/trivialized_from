use crate::metadata::attributes::into::IntoMetadata;
use crate::metadata::attributes::macro_transform::MacroTransformMetadata;
use crate::metadata::attributes::transform::TransformMetadata;
use crate::metadata::{as_name, ParseError};
use proc_macro2::{Ident, Span};
use quote::quote;
use std::hash::{Hash, Hasher};
use syn::spanned::Spanned;
use syn::Field;
use syn::__private::TokenStream2;

pub(crate) mod from;
pub(crate) mod into;
pub(crate) mod macro_transform;
pub(crate) mod transform;

#[derive(Clone)]
struct PathDetection {
    pub stringified: String,
    pub span: Span,
}

impl PartialEq for PathDetection {
    fn eq(&self, other: &Self) -> bool {
        (&self.stringified).eq(&other.stringified)
    }

    fn ne(&self, other: &Self) -> bool {
        (&self.stringified).ne(&other.stringified)
    }
}

impl Eq for PathDetection {}

impl Hash for PathDetection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.stringified.hash(state)
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        data.iter().for_each(|detection| detection.hash(state))
    }
}

pub(crate) enum FieldTransformation {
    Nothing,
    Into(IntoMetadata),
    Transform(TransformMetadata),
    MacroTransform(MacroTransformMetadata),
}

pub(crate) fn render_field_expression(field: &Field) -> Result<TokenStream2, Vec<ParseError>> {
    let name = match field.ident.as_ref() {
        None => {
            return Err(vec![ParseError {
                message: "Cannot find Identifier of this token".to_owned(),
                span: field.span().clone(),
            }])
        }
        Some(ident) => ident,
    };

    let field_transform_expression = render_field_transform_expression(name, field)?;

    Ok(quote! {#name: (#field_transform_expression)})
}

fn render_field_transform_expression(
    name: &Ident,
    field: &Field,
) -> Result<TokenStream2, Vec<ParseError>> {
    let type_path = if let syn::Type::Path(syn::TypePath { path: p, .. }) = &field.ty {
        as_name(&p.clone())
    } else {
        return Err(vec![ParseError {
            message: "Unable to extract type from this field".to_owned(),
            span: field.span().clone(),
        }]);
    };

    Ok(compute_transformations(field)?
        .iter()
        .fold(quote! {other.#name}, |tokens, transform| match transform {
            FieldTransformation::Nothing => tokens,
            FieldTransformation::Into(_) => {
                if type_path.ends_with("Vec") || type_path.ends_with("HashSet") {
                    quote! {#tokens.into_iter().map(Into::into).collect()}
                } else if type_path.ends_with("Option") {
                    quote! {#tokens.map(Into::into)}
                } else {
                    quote! {#tokens.into()}
                }
            }
            FieldTransformation::Transform(foo) => {
                let p = foo.transformation_path().unwrap();
                quote! {#p(#tokens)}
            }
            FieldTransformation::MacroTransform(foo) => {
                let p = foo.transformation_path().unwrap();
                quote! {#p!(#tokens)}
            }
        }))
}

fn compute_transformations(field: &Field) -> Result<Vec<FieldTransformation>, Vec<ParseError>> {
    let attrs = &field.attrs;
    let transformations: Vec<Result<FieldTransformation, ParseError>> = attrs
        .iter()
        .map(|a| {
            let into_result = IntoMetadata::maybe_from(a).map(|o| o.map(FieldTransformation::Into));

            match into_result {
                Err(p) => return Err(p),
                Ok(Some(transform)) => return Ok(transform),
                _ => (),
            };

            let transform_result =
                TransformMetadata::maybe_from(a).map(|o| o.map(FieldTransformation::Transform));

            match transform_result {
                Err(p) => return Err(p),
                Ok(Some(transform)) => return Ok(transform),
                _ => (),
            };

            let macro_transform_result = MacroTransformMetadata::maybe_from(a)
                .map(|o| o.map(FieldTransformation::MacroTransform));

            match macro_transform_result {
                Err(p) => Err(p),
                Ok(Some(transform)) => Ok(transform),
                _ => Ok(FieldTransformation::Nothing),
            }
        })
        .collect();

    let (impure_results, errors): (Vec<_>, Vec<_>) =
        transformations.into_iter().partition(Result::is_ok);

    let results = impure_results
        .into_iter()
        .filter_map(Result::ok)
        .filter(|r| !(matches!(r, FieldTransformation::Nothing)))
        .collect::<Vec<_>>();

    if errors.len() > 0 {
        Err(errors.into_iter().filter_map(Result::err).collect())
    } else if are_results_invalid(&results) {
        Err(vec![ParseError {
            message: "Field can be decorated at most one #[Into] and it must be the first one"
                .to_owned(),
            span: field.span().clone(),
        }])
    } else {
        Ok(results)
    }
}

fn are_results_invalid(results: &Vec<FieldTransformation>) -> bool {
    let into_count = results
        .iter()
        .filter(|r| matches!(r, FieldTransformation::Into(_)))
        .count();
    let is_first_not_an_into = results
        .first()
        .map(|r| !matches!(r, FieldTransformation::Into(_)))
        .unwrap_or(false);

    (is_first_not_an_into && into_count > 0) || into_count > 1
}
