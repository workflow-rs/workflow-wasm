use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use proc_macro_error::proc_macro_error;
mod callback;
use callback::Callback;


#[proc_macro]
#[proc_macro_error]
pub fn callback(input: TokenStream) -> TokenStream {
    let result =  parse_macro_input!(input as Callback);
    let ts = result.to_token_stream();
    //println!("\n===========> Callback <===========\n{}\n", ts.to_string());
    ts.into()
}
