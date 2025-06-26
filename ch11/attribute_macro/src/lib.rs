extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;

    let r#gen = quote! {
        fn #name() {
            println!("함수 '{}'가 호출되었습니다.", stringify!(#name));
            #block
        }
    };

    r#gen.into()
}
