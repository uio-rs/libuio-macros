use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as DeriveInput);

    // Generate new Rust code based on the transformed AST
    let expanded = quote! {
        #input
    };

    // Return the generated code
    let gen = TokenStream::from(expanded);
    println!("generated code: {:?}", gen);
    gen
}
