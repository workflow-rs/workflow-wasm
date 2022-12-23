
use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::ToTokens;
use proc_macro_error::proc_macro_error;
mod callback;
use callback::Callback;


/// 
/// creates [Callback](workflow_wasm::callback::Callback) instance 
/// by inspecting closure signature
/// 
/// ```
/// // include dependencies
/// use workflow_wasm::prelude::*;
/// ```
/// <div class="example-wrap compile_fail"><pre class="compile_fail" style="white-space:normal;font:inherit;">
///
///  ## **Warning**: Compile fails if arguments count of Closure variable is not equal to 1
///  Closure as variable can accept only single argument.
///  Otherwise it will give error as "closure is expected to take 1 argument"</pre></div>
/// 
///
/// ```compile_fail
/// // 2 arguments
/// let closure_as_variable = |arg1:bool, arg2:u16|{
///     //
/// };
/// let callback1 = callback!(closure_as_variable);
/// ```
/// 
/// ```compile_fail
/// // no arguments
/// let closure_as_variable = ||{
///     //
/// };
/// let callback1 = callback!(closure_as_variable);
/// ```
/// 
/// - ##### Closure as variable : will accept only 1 argument
///     ```
///     let closure_as_variable = |value:bool|{
///         //
///     };
///     let callback1 = callback!(closure_as_variable);
///     ```
///     Above code will create callback like this:
///     ```
///     let callback1 = Callback::new(closure_as_variable);
///     ```
///     ##### If you have closure variable with more or less than 1 argument use direct methods
///     - [Callback::new_with_args_0](workflow_wasm::callback::Callback#method.new_with_args_0)
///     - [Callback::new_with_args_2](workflow_wasm::callback::Callback#method.new_with_args_2)
///     - [Callback::new_with_args_3](workflow_wasm::callback::Callback#method.new_with_args_3)
///     - [Callback::new_with_args_4](workflow_wasm::callback::Callback#method.new_with_args_4)
///     - [Callback::new_with_args_5](workflow_wasm::callback::Callback#method.new_with_args_5)
///     - [Callback::new_with_args_6](workflow_wasm::callback::Callback#method.new_with_args_6)
///     - [Callback::new_with_args_7](workflow_wasm::callback::Callback#method.new_with_args_7)
///     - [Callback::new_with_args_8](workflow_wasm::callback::Callback#method.new_with_args_8)
///
///     
/// 
/// - ##### Direct Closure : can accept from 0-8 arguments
///     
///     ```
///     let callback2 = callback!(|value:bool|{
///         //
///     });
///     ```
///     Output will be as bellow:
///     ```
///     let callback2 = Callback::new_with_args_1(|value:bool|{
///         //
///     });
///     ```
/// - ##### Direct Closure : with 2 arguments
///     ```
///     let callback3 = callback!(|arg1:u16, value:bool|{
///         //
///     });
///     ```
///     Output will be as bellow:
///     ```
///     let callback3 = Callback::new_with_args_2(|arg1:u16, value:bool|{
///        //
///     });
///     ```
/// 
#[proc_macro]
#[proc_macro_error]
pub fn callback(input: TokenStream) -> TokenStream {
    let result =  parse_macro_input!(input as Callback);
    let ts = result.to_token_stream();
    //println!("\n===========> Callback <===========\n{}\n", ts.to_string());
    ts.into()
}
