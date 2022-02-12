use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod {{macro_name}};
use crate::{{macro_name}}::analyze;
use crate::{{macro_name}}::codegen;
use crate::{{macro_name}}::lower;
use crate::{{macro_name}}::parse;

#[proc_macro]
#[proc_macro_error]
pub fn {{macro_name}}(ts: TokenStream) -> TokenStream {
    let ast = parse::parse(ts.clone().into());
    let model = analyze::analyze(ast);
    let ir = lower::lower(model);
    let _ = codegen::codegen(ir);
    TokenStream::new()
}
