extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_world(&ast)
}

fn impl_hello_world(ast: &syn::DeriveInput) -> TokenStream {

    let name = &ast.ident;
    let r#gen = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! I am a {}", stringify!(#name));
            }
        }
    };

    r#gen.into()
}


