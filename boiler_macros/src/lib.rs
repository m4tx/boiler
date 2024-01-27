use proc_macro2_diagnostics::SpanDiagnosticExt;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FunctionMeta)]
pub fn derive_function_meta(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();

    if !name_str.ends_with("Action") && !name_str.ends_with("Detector") {
        return name
            .span()
            .error("FunctionMeta can only be derived for types ending with 'Action' or 'Detector'")
            .emit_as_item_tokens()
            .into();
    }

    let function_name = name_str.replace("Action", "").replace("Detector", "");

    let docs = get_comment(&input.attrs);

    let boiler = crate_name("boiler_core").expect("boiler_core is not present in `Cargo.toml`");
    let crate_ident = match boiler {
        FoundCrate::Itself => {
            quote! { crate }
        }
        FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
    };

    let expanded = quote! {
        impl #crate_ident::function_meta::FunctionMeta for #name {
            fn name(&self) -> &'static str { #function_name }
            fn description(&self) -> &'static str { #docs }
            fn default_enabled(&self) -> bool { true }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn get_comment(attrs: &[syn::Attribute]) -> String {
    let string_literals = attrs
        .iter()
        .filter_map(|attr| match attr.meta {
            syn::Meta::NameValue(ref name_value) if name_value.path.is_ident("doc") => {
                match &name_value.value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(lit_str),
                        ..
                    }) => Some(lit_str.value()),
                    _ => None,
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let docs_trimmed: Vec<_> = string_literals
        .iter()
        .flat_map(|literal| literal.split('\n').collect::<Vec<_>>())
        .map(|line| line.trim().to_owned())
        .collect();

    docs_trimmed.join("\n")
}
