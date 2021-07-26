use crate::metadata::attributes::PathDetection;
use crate::metadata::{as_name, ParseError};
use syn::spanned::Spanned;
use syn::{Attribute, Meta, NestedMeta};

pub(crate) struct MacroTransformMetadata {
    path_detection: PathDetection,
}

//TODO: refactor, see FromMetadata
impl MacroTransformMetadata {
    pub(crate) fn maybe_from(attr: &Attribute) -> Result<Option<Self>, ParseError> {
        if as_name(&attr.path).ne("MacroTransform") {
            return Ok(None);
        }

        let meta = attr.parse_meta().map_err(|e| {
            ParseError {
                message: format!("Unable to successfully parse this attribute because \"{}\". Expected format is: #[MacroTransform(path)]", e),
                span: attr.span().clone(),
            }
        })?;

        let meta_list = match meta {
            Meta::List(list) => Ok(list),
            Meta::Path(path) => Err(ParseError {
                message: format!("#[MacroTransform] attribute does not support Path format. Expected format is: #[MacroTransform(path)]"),
                span: path.span().clone()
            }),
            Meta::NameValue(name_value) => Err(ParseError {
                message: format!("#[MacroTransform] attribute does not support NameValue format. Expected format is: #[MacroTransform(path)]"),
                span: name_value.span().clone()
            })
        }?;

        let extracted_path: Option<_> = meta_list.nested.first().map(|nested_meta| {
            let meta = match nested_meta {
                NestedMeta::Meta(meta) => Ok(meta),
                NestedMeta::Lit(lit) => Err(ParseError {
                    message: format!("Literal NestedMeta detected in #[MacroTransform] MetaList. Expected format is: #[MacroTransform(path)]"),
                    span: lit.span().clone()
                })
            }?;

            match meta {
                Meta::Path(ref path) => Ok(PathDetection {
                    stringified: as_name(&path),
                    span: path.span().clone()
                }),
                _ => Err(ParseError {
                    message: format!("NestedMeta Path is needed in #[MacroTransform] MetaList. Expected format is: #[MacroTransform(path)]"),
                    span: meta.span().clone()
                })
            }
        });

        extracted_path
            .unwrap_or_else(|| {
                Err(ParseError {
                    message: "Unable to retrieve any Path from this #[MacroTransform]".to_owned(),
                    span: meta_list.clone().span(),
                })
            })
            .map(|res| {
                Some(MacroTransformMetadata {
                    path_detection: res,
                })
            })
    }

    pub(crate) fn transformation_path(&self) -> Result<syn::Path, ParseError> {
        syn::parse_str::<syn::Path>(&self.path_detection.stringified).map_err(|e| ParseError {
            message: format!("Unable to parse type from this token: {}", e),
            span: self.path_detection.span.clone(),
        })
    }
}
