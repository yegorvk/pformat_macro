use crate::data::{Args, FormatArgs, FormatStr, PFormatArgs, PFormatArgsAST};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Lit, Token};

impl Parse for FormatStr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format: Expr = input.parse()?;
        if let Expr::Lit(format) = format {
            if let Lit::Str(format) = format.lit {
                return Ok(format.into());
            }

            Err(syn::Error::new(
                format.span(),
                "format must be a constant string literal",
            ))
        } else {
            Err(syn::Error::new(
                format.span(),
                "format must be a constant string literal",
            ))
        }
    }
}

impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Args = input.parse_terminated(Expr::parse)?;
        Ok(Some(args).into())
    }
}

impl Parse for PFormatArgsAST {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let str: FormatStr = input.parse()?;
        let args = if input.peek(Token![,]) {
            //Format arguments are present.
            input.parse::<Token![,]>()?;
            let args: FormatArgs = input.parse()?;
            args
        } else {
            //No format arguments.
            None.into()
        };

        Ok(PFormatArgsAST { str, args })
    }
}

impl Parse for PFormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: PFormatArgsAST = input.parse()?;
        ast.try_into()
    }
}
