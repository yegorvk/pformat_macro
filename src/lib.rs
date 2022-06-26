use crate::generator::expand;
use syn::{parse_macro_input, Expr};

mod generator;
mod parser;

pub(crate) struct PFormatArgs {
    pub pieces: Vec<String>,
    pub args: Vec<Expr>,
}

#[proc_macro]
pub fn pformat_args(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(_input as PFormatArgs);
    proc_macro::TokenStream::from(expand(input))
}
