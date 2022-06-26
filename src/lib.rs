use crate::data::PFormatArgs;
use crate::generator::expand_macro;
use syn::parse_macro_input;

mod data;
mod generator;
mod parser;

#[proc_macro]
pub fn pformat_args(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(_input as PFormatArgs);
    proc_macro::TokenStream::from(expand_macro(input))
}
