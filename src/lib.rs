extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(DummyAttr, attributes(into))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

//TODO: maybe this macro can become a deriveMacro with helper attributes
#[proc_macro_attribute]
pub fn from(attr: TokenStream, item: TokenStream) -> TokenStream {
    let types = extract_types_from(&attr);
    let structure = syn::parse::<syn::ItemStruct>(item.clone()).unwrap();
    let struct_name = &structure.ident;
    let fields = if let syn::Fields::Named(ref named) = structure.fields {
        named
    } else {
        unimplemented!("Need fields!")
    };
    let converted_fields: Vec<_> = fields.named.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        let type_segments = if let syn::Type::Path(syn::TypePath {path: something, ..}) = &f.ty {
            &something.segments
        } else {
            unimplemented!("Need a TypePath!")
        };
        let attrs = &f.attrs;
        if attrs.iter().any(|a| a.path.segments.last().unwrap().ident.to_string().eq("into")) {
            if type_segments.last().as_ref().unwrap().ident.to_string().eq("Option") {
                quote!{#name: other.#name.map(Into::into)}
            } else if type_segments.last().as_ref().unwrap().ident.to_string().eq("Vec") {
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
    
    let out = quote! {
        #structure

        #(#impl_blocks)*
    };

    out.into()
}

fn extract_types_from(types: &TokenStream) -> Vec<syn::Type> {
    types.to_string().split(",").map(|s| s.trim()).map(|t| syn::parse_str(t).unwrap()).collect()
}