use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_macro_input};

#[proc_macro]
pub fn make_json(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Expr);
    let r#gen = quote! {
        {
            let value = #parsed;
            serde_json::to_string(&value).unwrap()
        }
    };

    r#gen.into()
}
