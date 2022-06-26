use crate::PFormatArgs;
use proc_macro2::{Span, TokenStream};
use proc_quote::quote as quote2;
use syn::{Expr, Ident};

trait IdentExt {
    fn from_str(string: &str) -> Ident {
        Ident::new(string, Span::call_site())
    }
}

impl IdentExt for Ident {}

trait Generate {
    fn generate(&self) -> TokenStream;
}

#[repr(transparent)]
struct Argument {
    position: usize,
}

impl Argument {
    fn new(position: usize) -> Argument {
        Argument { position }
    }

    fn name_ident(&self) -> Ident {
        Ident::from_str(&format!("arg{}", self.position))
    }

    fn type_ident(&self) -> Ident {
        Ident::from_str(&format!("T{}", self.position))
    }
}

#[repr(transparent)]
struct Arguments<'a> {
    args: &'a [Expr],
}

impl<'a> Arguments<'a> {
    fn new(args: &'a [Expr]) -> Arguments<'a> {
        Arguments { args }
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn name_idents(&self) -> impl Iterator<Item = Ident> {
        (0..self.len()).map(|i| Argument::new(i).name_ident())
    }

    fn type_idents(&self) -> impl Iterator<Item = Ident> {
        (0..self.len()).map(|i| Argument::new(i).type_ident())
    }

    fn as_slice(&self) -> &[Expr] {
        self.args
    }
}

#[repr(transparent)]
struct StructGenericArguments<'a> {
    arguments: &'a Arguments<'a>,
}

impl<'a> StructGenericArguments<'a> {
    fn new(arguments: &'a Arguments) -> StructGenericArguments<'a> {
        StructGenericArguments { arguments }
    }

    fn generics(&self) -> impl Iterator<Item = Ident> {
        self.arguments.type_idents()
    }

    fn as_generic_params(&'a self) -> impl Generate + 'a {
        struct ConstrainedStructGenerics<'a>(&'a StructGenericArguments<'a>);

        impl<'a> Generate for ConstrainedStructGenerics<'a> {
            fn generate(&self) -> TokenStream {
                let generics = self.0.generics();

                quote2! {
                    #(#generics: Display, )*
                }
            }
        }

        ConstrainedStructGenerics::<'a>(self)
    }
}

impl Generate for StructGenericArguments<'_> {
    fn generate(&self) -> TokenStream {
        let generics = self.generics();

        quote2! {
            #(#generics, )*
        }
    }
}

#[repr(transparent)]
struct StructProperties<'a> {
    arguments: &'a Arguments<'a>,
}

impl<'a> StructProperties<'a> {
    fn new(arguments: &'a Arguments) -> StructProperties<'a> {
        StructProperties { arguments }
    }
}

impl Generate for StructProperties<'_> {
    fn generate(&self) -> TokenStream {
        let names = self.arguments.name_idents();
        let types = self.arguments.type_idents();

        quote2! {
            #(#names: #types,)*
        }
    }
}

struct StructDeclaration<'a> {
    name: &'a Ident,
    arguments: &'a Arguments<'a>,
}

impl<'a> StructDeclaration<'a> {
    fn new(name: &'a Ident, arguments: &'a Arguments) -> StructDeclaration<'a> {
        StructDeclaration { name, arguments }
    }
}

impl Generate for StructDeclaration<'_> {
    fn generate(&self) -> TokenStream {
        let name = self.name;
        let properties = StructProperties::new(self.arguments).generate();
        let generics = StructGenericArguments::new(self.arguments)
            .as_generic_params()
            .generate();

        quote2! {
            struct #name<#generics> {
                #properties
            };
        }
    }
}

struct DisplayFunc<'a> {
    pieces: &'a [String],
    arguments: &'a Arguments<'a>,
}

impl<'a> DisplayFunc<'a> {
    fn new(pieces: &'a [String], arguments: &'a Arguments) -> DisplayFunc<'a> {
        DisplayFunc { pieces, arguments }
    }
}

impl Generate for DisplayFunc<'_> {
    fn generate(&self) -> TokenStream {
        let prefix = &self.pieces[0];
        let pieces = &self.pieces[1..];
        let arguments = self.arguments.name_idents();

        quote2! {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.write_str(#prefix)?;

                #(
                    f.write_fmt(format_args!("{}", &self.#arguments))?;
                    f.write_str(#pieces)?;
                )*

                Ok(())
            }
        }
    }
}

struct DisplayImplDeclaration<'a> {
    struct_name: &'a Ident,
    pieces: &'a [String],
    arguments: &'a Arguments<'a>,
}

impl<'a> DisplayImplDeclaration<'a> {
    fn new(
        struct_name: &'a Ident,
        pieces: &'a [String],
        arguments: &'a Arguments,
    ) -> DisplayImplDeclaration<'a> {
        DisplayImplDeclaration {
            struct_name,
            pieces,
            arguments,
        }
    }
}

impl Generate for DisplayImplDeclaration<'_> {
    fn generate(&self) -> TokenStream {
        let name = self.struct_name;
        let display_func = DisplayFunc::new(self.pieces, self.arguments).generate();
        let generic_arguments = StructGenericArguments::new(self.arguments).generate();
        let generic_params = StructGenericArguments::new(self.arguments)
            .as_generic_params()
            .generate();

        quote2! {
            impl<#generic_params> Display for #name<#generic_arguments> {
                #display_func
            }
        }
    }
}

struct StructInitializer<'a> {
    name: &'a Ident,
    arguments: &'a Arguments<'a>,
}

impl<'a> StructInitializer<'a> {
    fn new(name: &'a Ident, arguments: &'a Arguments) -> StructInitializer<'a> {
        StructInitializer { name, arguments }
    }
}

impl Generate for StructInitializer<'_> {
    fn generate(&self) -> TokenStream {
        let name = self.name;
        let argument_names = self.arguments.name_idents();
        let argument_values = self.arguments.as_slice();

        quote2! {
            #name {
                #(#argument_names: #argument_values, )*
            }
        }
    }
}

struct MacroInput<'a> {
    struct_name: Ident,
    pieces: &'a [String],
    arguments: Arguments<'a>,
}

impl<'a> MacroInput<'a> {
    fn new(struct_name: Ident, pieces: &'a [String], arguments: Arguments<'a>) -> MacroInput<'a> {
        MacroInput {
            struct_name,
            pieces,
            arguments,
        }
    }
}

impl<'a> From<&'a PFormatArgs> for MacroInput<'a> {
    fn from(value: &'a PFormatArgs) -> MacroInput<'a> {
        MacroInput::new(
            Ident::from_str("CompiledFormat"),
            &value.pieces,
            Arguments::new(&value.args),
        )
    }
}

#[repr(transparent)]
struct ExpandedMacro<'a> {
    input: MacroInput<'a>,
}

impl ExpandedMacro<'_> {
    fn new(input: &PFormatArgs) -> ExpandedMacro {
        ExpandedMacro {
            input: input.into(),
        }
    }

    fn struct_declaration(&self) -> TokenStream {
        StructDeclaration::new(&self.input.struct_name, &self.input.arguments).generate()
    }

    fn display_impl_declaration(&self) -> TokenStream {
        DisplayImplDeclaration::new(
            &self.input.struct_name,
            self.input.pieces,
            &self.input.arguments,
        )
        .generate()
    }

    fn struct_initializer(&self) -> TokenStream {
        StructInitializer::new(&self.input.struct_name, &self.input.arguments).generate()
    }
}

impl Generate for ExpandedMacro<'_> {
    fn generate(&self) -> TokenStream {
        let struct_decl = self.struct_declaration();
        let display_impl_decl = self.display_impl_declaration();
        let struct_initializer = self.struct_initializer();

        quote2! {
            {
                use std::fmt::Display;
                use std::fmt::Formatter;
                use std::fmt::Result;
                use std::format_args;

                #struct_decl
                #display_impl_decl
                #struct_initializer
            }
        }
    }
}

pub(crate) fn expand(input: PFormatArgs) -> TokenStream {
    ExpandedMacro::new(&input).generate()
}
