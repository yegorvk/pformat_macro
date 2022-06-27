//! Rust standard library has [`format_args!`] macro that lets
//! you combine format string and its arguments into the [`Arguments`] structure
//! without allocations in cost of directly referencing parts of format string,
//! so the result must be used immediately (to satisfy borrow checker).
//!
//! [`format_args!`]: https://doc.rust-lang.org/std/macro.format_args.html
//! [`Arguments`]: https://doc.rust-lang.org/stable/std/fmt/struct.Arguments.html
//!
//! This crates offers `pformat_args!` macro that returns `impl Display` instance
//! that can be used just like any normal structure holding provided format arguments.
//!
//! <br/>
//!
//! # Example
//!
//! The usage is almost the same as  [`format_args!`] except that all `{}`
//! placeholders must be empty.
//!
//! ```
//! use pformat_macro::pformat_args;
//!
//! let result_str = pformat_args!("1 + 1 = {}", 3);
//! println!("{}", result_str) //prints 1 + 1 = 3
//! ```

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
