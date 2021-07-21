use crate::metadata::{as_name, ParseError};
use syn::spanned::Spanned;
use syn::{Attribute, Meta};

pub(crate) struct IntoMetadata {}

#[allow(unused)]
impl IntoMetadata {
    pub(crate) fn maybe_from(attr: &Attribute) -> Result<Option<Self>, Vec<ParseError>> {
        if as_name(&attr.path).ne("Into") {
            return Ok(None);
        }

        let meta = attr.parse_meta().map_err(|e| {
            vec![ParseError {
                message: format!("Unable to successfully parse this attribute because \"{}\". Expected format is: #[Into]", e),
                span: attr.span().clone(),
            }]
        })?;

        match meta {
            Meta::Path(_) => Ok(Some(IntoMetadata{})),
            Meta::List(list) => Err(vec![ParseError {
                message: format!("#[Into] attribute does not support List format. Expected format is: #[Into]"),
                span: list.span().clone()
            }]),
            Meta::NameValue(name_value) => Err(vec![ParseError {
                message: format!("#[From] attribute does not support NameValue format. Expected format is: #[Into]"),
                span: name_value.span().clone()
            }])
        }
    }
}
