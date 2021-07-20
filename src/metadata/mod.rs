use proc_macro2::Span;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use syn::spanned::Spanned;
use syn::Path;
use syn::{Attribute, Meta, NestedMeta};

pub(crate) fn as_name(p: &Path) -> String {
    p.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

#[derive(Clone)]
pub(crate) struct ParseError {
    pub message: String,
    pub span: Span,
}

#[derive(Clone)]
pub(crate) struct TypeDetection {
    pub stringified: String,
    pub span: Span,
}

impl PartialEq for TypeDetection {
    fn eq(&self, other: &Self) -> bool {
        (&self.stringified).eq(&other.stringified)
    }

    fn ne(&self, other: &Self) -> bool {
        (&self.stringified).ne(&other.stringified)
    }
}

impl Eq for TypeDetection {}

impl Hash for TypeDetection {
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

#[derive(Default)]
pub(crate) struct FromMetadata {
    pub type_detections: HashSet<TypeDetection>,
}

impl FromMetadata {
    pub(crate) fn maybe_from(attr: &Attribute) -> Result<Option<Self>, Vec<ParseError>> {
        if as_name(&attr.path).ne("From") {
            return Ok(None);
        }

        let meta = attr.parse_meta().map_err(|e| {
            vec![ParseError {
                message: format!("Unable to successfully parse this attribute because \"{}\". Expected format is: #[From(Type1,...,TypeN)]", e),
                span: attr.span().clone(),
            }]
        })?;

        let meta_list = match meta {
            Meta::List(list) => Ok(list),
            Meta::Path(path) => Err(vec![ParseError {
                message: format!("#[From] attribute does not support Path format. Expected format is: #[From(Type1,...,TypeN)]"),
                span: path.span().clone()
            }]),
            Meta::NameValue(name_value) => Err(vec![ParseError {
                message: format!("#[From] attribute does not support NameValue format. Expected format is: #[From(Type1,...,TypeN)]"),
                span: name_value.span().clone()
            }])
        }?;

        let extracted_types = meta_list.nested.iter().map(|nested_meta| {
            let meta = match nested_meta {
                NestedMeta::Meta(meta) => Ok(meta),
                NestedMeta::Lit(lit) => Err(ParseError {
                    message: format!("Literal NestedMeta detected in #[From] MetaList. Expected format is: #[From(Type1,...,TypeN)]"),
                    span: lit.span().clone()
                })
            }?;

            match meta {
                Meta::Path(ref path) => Ok(TypeDetection {
                    stringified: as_name(&path),
                    span: path.span().clone()
                }),
                _ => Err(ParseError {
                    message: format!("NestedMeta Path is needed in #[From] MetaList. Expected format is: #[From(Type1,...,TypeN)]"),
                    span: meta.span().clone()
                })
            }
        }).collect::<Vec<Result<TypeDetection, ParseError>>>();

        let (types, errors): (
            Vec<Result<TypeDetection, ParseError>>,
            Vec<Result<TypeDetection, ParseError>>,
        ) = extracted_types.into_iter().partition(Result::is_ok);

        if errors.len() > 0 {
            return Err(errors.into_iter().filter_map(Result::err).collect());
        }

        Ok(Some(FromMetadata {
            type_detections: types
                .into_iter()
                .filter_map(Result::ok)
                .collect::<HashSet<TypeDetection>>(),
        }))
    }

    pub(crate) fn types(&self) -> Result<Vec<syn::Type>, Vec<ParseError>> {
        let result_types = self
            .type_detections
            .iter()
            .map(|detection| {
                syn::parse_str::<syn::Type>(&detection.stringified).map_err(|e| ParseError {
                    message: format!("Unable to parse type from this token: {}", e),
                    span: detection.span.clone(),
                })
            })
            .collect::<Vec<Result<syn::Type, ParseError>>>();

        let (types, errors): (
            Vec<Result<syn::Type, ParseError>>,
            Vec<Result<syn::Type, ParseError>>,
        ) = result_types.into_iter().partition(Result::is_ok);

        if errors.len() > 0 {
            return Err(errors.into_iter().filter_map(Result::err).collect());
        }

        Ok(types.into_iter().filter_map(Result::ok).collect())
    }

    pub(crate) fn merge(self, other: Self) -> Self {
        Self {
            type_detections: self
                .type_detections
                .into_iter()
                .chain(other.type_detections)
                .collect(),
        }
    }
}

/*
enum FieldMetadata {
    Transform(TransformMetadata),
    Into(IntoMetadata),
}

struct TransformMetadata {
    pub transformation_name: String,
}

struct IntoMetadata {}
*/
