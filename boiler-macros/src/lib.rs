use proc_macro2_diagnostics::SpanDiagnosticExt;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ActionMeta)]
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    if !name.to_string().ends_with("Action") {
        return name
            .span()
            .error("ActionMeta can only be derived for types ending with 'Action'")
            .emit_as_item_tokens()
            .into();
    }

    let action_name = name.to_string().replace("Action", "");

    let docs = get_comment(&input.attrs);

    let boiler = crate_name("boiler").expect("boiler is not present in `Cargo.toml`");
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
        impl #crate_ident::actions::ActionMeta for #name {
            fn name(&self) -> &'static str { #action_name }
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
