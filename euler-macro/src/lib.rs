use attribute::problem_inner;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

mod attribute;

use crate::attribute::Args;

#[proc_macro_attribute]
pub fn problem(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let f = parse_macro_input!(input as ItemFn);

    problem_inner(args, f).into()
}
