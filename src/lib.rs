use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(item as ItemFn);

    // Check if the function is async
    if input.sig.asyncness.is_none() {
        panic!("The function must be async");
    }

    // Check if the function returns ()
    if let ReturnType::Type(_, _) = &input.sig.output {
        panic!("The function must return ()");
    }

    // Generate new Rust code based on the transformed AST
    let expanded = quote! {
        async fn __internal_main() {
            #input.block
        }

        fn main() {
            // First we need to create a new thread pool to execute on.
            let pool = libuio::executor::ThreadPoolBuilder::new()
                .name_prefix("executor")
                .create()
                .expect("Failed to configure thread pool.");

            // Now we spawn our main async task, which will drive any/all async operations needed by our
            // application.
            pool.spawn_ok(__internal_main());

            pool.wait();
        }
    };

    // Return the generated code
    let gen = TokenStream::from(expanded);
    println!("generated code: {:?}", gen);
    gen
}
