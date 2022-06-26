use crate::data::PFormatArgs;
use proc_macro2::{Span, TokenStream};
use proc_quote::quote as quote2;
use syn::{Expr, GenericParam, Ident, TypeParam};

fn ident_token(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

fn arg_type(i: usize) -> Ident {
    ident_token(&format!("T{}", i))
}

fn arg_name(i: usize) -> Ident {
    ident_token(&format!("arg{}", i))
}

fn generic_params(count: usize) -> impl Iterator<Item = GenericParam> {
    (0..count).map(|i| TypeParam::from(arg_type(i)).into())
}

fn arg_declaration(i: usize) -> TokenStream {
    let name = arg_name(i);
    let a_type = arg_type(i);

    quote2! {
        #name: #a_type
    }
}

fn args(count: usize) -> TokenStream {
    let args = (0..count).map(arg_declaration);

    quote2! {
        #(#args),*
    }
}

fn arg_init(i: usize, initializer: &Expr) -> TokenStream {
    let name = arg_name(i);
    quote2! {
        #name: #initializer
    }
}

fn args_init(args: &[Expr]) -> TokenStream {
    let args = args.iter().enumerate().map(|(i, x)| arg_init(i, x));

    quote2! {
        #(#args, )*
    }
}

fn arg_names(count: usize) -> impl Iterator<Item = Ident> {
    (0..count).map(arg_name)
}

fn expand_macro_internal(name: &str, input: PFormatArgs) -> TokenStream {
    let name = ident_token(name);
    let pieces = input.pieces;
    let args = args(input.args.len());
    let arg_names = arg_names(input.args.len());
    let args_init = args_init(&input.args);
    let arg_count = input.args.len();

    let (generic_params, generic_params_copy) = (
        generic_params(input.args.len()),
        generic_params(input.args.len()),
    );

    let generic_params = quote2! {
        #(#generic_params, )*
    };

    let generic_params_with_constraints = quote2! {
        #(#generic_params_copy: Display, )*
    };

    let indices = 1..(arg_count + 1);

    let pieces = if pieces.is_empty() {
        vec![String::new()]
    } else {
        pieces
    };

    quote2! {
        {
            use std::fmt::Display;
            use std::fmt::Formatter;
            use std::fmt::Result;
            use std::format_args;

            struct #name<#generic_params_with_constraints const N: usize> {
                pieces: [&'static str; N],
                #args
            }

            impl<#generic_params_with_constraints const N: usize> Display for #name<#generic_params N> {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    f.write_str(self.pieces[0])?;

                    #(f.write_fmt(format_args!("{}", self.#arg_names))?;
                      f.write_str(self.pieces[#indices])?;
                    )*

                    Ok(())
                }
            }

            #name {
                pieces: [#(#pieces),*],
                #args_init
            }
        }
    }
}

pub fn expand_macro(input: PFormatArgs) -> TokenStream {
    expand_macro_internal("CompiledFormat", input)
}
