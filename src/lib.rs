use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);
    let block = input.block;
    let output = input.sig.output;

    // Check if the function is async
    if input.sig.asyncness.is_none() {
        panic!("The function must be async");
    }

    // Generate new Rust code based on the transformed AST
    let expanded = quote! {
        // Create new async function so we can replace the main function. Ensuring we handle the
        // output type and proper block representation.
        async fn __internal_main() #output #block

        // Create the new main function with the proper output type to satisfy the original main
        // function.
        fn main() #output {
            // First we need to create a new thread pool to execute on.
            libuio::executor::ThreadPoolBuilder::new()
                .name_prefix("libuio-executor")
                .create()
                .expect("Failed to configure thread pool.");

            // Now we spawn our main async task, which will drive any/all async operations needed by our
            // application.
            libuio::executor::block_on(__internal_main())
        }
    };

    // Return the generated code
    let gen = TokenStream::from(expanded);
    println!("generated code: {}", gen);
    gen
}
