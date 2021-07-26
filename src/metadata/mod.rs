pub(crate) mod attributes;

use proc_macro2::Span;
use syn::Path;

pub(crate) fn as_name(p: &Path) -> String {
    p.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

#[derive(Clone, Debug)]
pub(crate) struct ParseError {
    pub message: String,
    pub span: Span,
}
