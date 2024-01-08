use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
pub fn solution(args: TokenStream, input: TokenStream) -> TokenStream {
    assert!(args.is_empty());

    let mut calculate_solution = parse_macro_input!(input as ItemFn);
    calculate_solution.sig.ident =
        Ident::new("calculate_solution", calculate_solution.sig.ident.span());

    let tokens = quote! {
        mod data;

        #calculate_solution

        fn main() {
            let now = std::time::Instant::now();
            let result = calculate_solution(data::INPUT);
            let elapsed = now.elapsed();
            println!("Solution is: {:?}", result);
            if elapsed.as_millis() > 0 {
                println!("Time: {}ms", elapsed.as_millis());
            } else {
                println!("Time: {}μs", elapsed.as_micros());
            }
      }
    };
    TokenStream::from(tokens)
}
