use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Expr, LitStr, Token};

#[repr(transparent)]
pub struct FormatStr {
    format: LitStr,
}

impl FormatStr {
    pub fn split_into_pieces<T: FromIterator<String>>(&self) -> T {
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

pub type Args = Punctuated<Expr, Token![,]>;

#[repr(transparent)]
pub struct FormatArgs {
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

pub struct PFormatArgsAST {
    pub str: FormatStr,
    pub args: FormatArgs,
}

pub struct PFormatArgs {
    pub pieces: Vec<String>,
    pub args: Vec<Expr>,
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
