use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item);
    let name = &input.sig.ident;
    let block = &input.block;
    let output = &input.sig.output;

    let r#gen = quote! {
        fn #name() #output {
            println!("함수 '{}'가 호출되었습니다.", stringify!(#name));
            #block
        }
    };

    r#gen.into()
}
