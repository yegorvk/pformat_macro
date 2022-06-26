use crate::PFormatArgs;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, Lit, LitStr, Token};

#[repr(transparent)]
struct FormatStr {
    format: LitStr,
}

impl FormatStr {
    fn split_into_pieces<T: FromIterator<String>>(&self) -> T {
        self.format
            .value()
            .split(&"{}")
            .map(|x| x.to_string())
            .collect()
    }
}

impl From<LitStr> for FormatStr {
    fn from(format: LitStr) -> Self {
        FormatStr { format }
    }
}

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

type Args = Punctuated<Expr, Token![,]>;

#[repr(transparent)]
struct FormatArgs {
    args: Option<Args>,
}

impl FormatArgs {
    fn len(&self) -> usize {
        match &self.args {
            Some(args) => args.len(),
            None => 0,
        }
    }

    fn span(&self) -> Option<Span> {
        self.args.as_ref().map(|args| args.span())
    }

    fn span_or_default(&self) -> Span {
        self.span().unwrap_or_else(Span::call_site)
    }
}

impl IntoIterator for FormatArgs {
    type Item = Expr;
    type IntoIter = <Args as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self.args {
            Some(args) => args.into_iter(),
            None => Args::default().into_iter(),
        }
    }
}

impl From<Option<Args>> for FormatArgs {
    fn from(args: Option<Args>) -> Self {
        FormatArgs { args }
    }
}

impl Parse for FormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Args = input.parse_terminated(Expr::parse)?;
        Ok(Some(args).into())
    }
}

struct PFormatArgsAST {
    str: FormatStr,
    args: FormatArgs,
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

impl TryFrom<PFormatArgsAST> for PFormatArgs {
    type Error = syn::Error;

    fn try_from(value: PFormatArgsAST) -> Result<Self, Self::Error> {
        let pieces: Vec<String> = value.str.split_into_pieces();

        if pieces.len() - 1 < value.args.len() {
            Err(syn::Error::new(
                value.args.span_or_default(),
                "too many format arguments passed (more than parameters)",
            ))
        } else if pieces.len() - 1 > value.args.len() {
            Err(syn::Error::new(
                value.args.span_or_default(),
                "too few format arguments passed (fewer than parameters)",
            ))
        } else {
            Ok(PFormatArgs {
                pieces,
                args: value.args.into_iter().collect(),
            })
        }
    }
}

impl Parse for PFormatArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ast: PFormatArgsAST = input.parse()?;
        ast.try_into()
    }
}
