//! Macro to extract data from deeply nested types representing GraphQL results
//!
//! # Suggested workflow
//!
//! 1. Generate query types using [cynic] and its [generator]
//! 1. Use [insta] to define an inline snapshot test so that the query string is visible in the
//!    module that defines the query types
//! 1. Define an `extract` function that takes the root query type and returns the data of interest
//! 1. Inside `extract`, use [`extract!`](crate::extract) as `extract!(data => { ... })`
//! 1. Inside the curly braces, past the query string from the snapshot test above
//! 1. Change all node names from `camelCase` to `snake_case`
//! 1. Add `?` after the nodes that are nullable
//! 1. Add `[]` after the nodes that are iterable
//!
//! [cynic]: https://cynic-rs.dev/
//! [generator]: https://generator.cynic-rs.dev/
//! [insta]: https://insta.rs/

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens as _};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned as _;
use syn::token::{self, Brace};
use syn::{braced, bracketed, parse_macro_input, parse_quote, Error, Ident, Token};

/// See the top-level [`crate`] doc for a description.
#[proc_macro]
pub fn extract(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let root = parse_macro_input!(input as Root);
    let stmt = root.generate_extract();
    stmt.into()
}

struct Root {
    expr: syn::Expr,
    nested: Nested,
}

struct Node {
    ident: Ident,
    optional: bool,
    iterable: bool,
    nested: Option<Nested>,
}

enum Nested {
    Nodes(Vec<Node>),
    Variant(Variant),
}

struct Variant {
    path: syn::Path,
    nodes: Vec<Node>,
}

//=================================================================================================
// Parsing
//=================================================================================================

impl Parse for Root {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let nested = input.parse()?;
        Ok(Self { expr, nested })
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut self_ = Self {
            ident: input.parse()?,
            optional: false,
            iterable: false,
            nested: None,
        };

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                break; // There's another field to be parsed
            } else if lookahead.peek(Token![?]) {
                let question: Token![?] = input.parse()?;
                if self_.optional {
                    return Err(Error::new_spanned(
                        question,
                        "Can't have two `?` for the same node",
                    ));
                }
                self_.optional = true;
            } else if lookahead.peek(token::Bracket) {
                let content;
                let bracket = bracketed!(content in input);
                if self_.iterable {
                    return Err(Error::new(
                        bracket.span.span(),
                        "Can't have two `[]` for the same node",
                    ));
                }
                if !content.is_empty() {
                    return Err(Error::new(
                        bracket.span.span(),
                        "Only empty brackets allowed",
                    ));
                }
                self_.iterable = true;
            } else if lookahead.peek(token::Brace) {
                let nested = input.parse()?;
                self_.nested = Some(nested);
                break; // Everything after the closing brace is ignored
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(self_)
    }
}

impl Node {
    fn within_braces(brace: Brace, content: ParseStream) -> syn::Result<Vec<Self>> {
        let mut nodes = vec![];
        while !content.is_empty() {
            let lookahead = content.lookahead1();
            if lookahead.peek(Token![...]) {
                return Err(Error::new(
                    brace.span.span(),
                    "Nodes can't be mixed with '... on Variant' matches",
                ));
            }
            nodes.push(content.parse()?);
        }
        if nodes.is_empty() {
            return Err(Error::new(
                brace.span.span(),
                "Empty braces; must have at least one node",
            ));
        }
        Ok(nodes)
    }
}

impl Parse for Nested {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = braced!(content in input);

        let lookahead = content.lookahead1();
        Ok(if lookahead.peek(Token![...]) {
            let var = Self::Variant(content.parse()?);
            if !content.is_empty() {
                return Err(Error::new(
                    brace.span.span(),
                    "Only a single '... on Variant' match is supported within the same braces",
                ));
            }
            var
        } else {
            Self::Nodes(Node::within_braces(brace, &content)?)
        })
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![...]>()?;
        let on: Ident = input.parse()?;
        if on != "on" {
            return Err(Error::new(on.span(), "Expected 'on'"));
        }
        let path = input.parse()?;
        let content;
        let brace = braced!(content in input);
        Ok(Self {
            path,
            nodes: Node::within_braces(brace, &content)?,
        })
    }
}

//=================================================================================================
// Generation
//=================================================================================================

impl Root {
    fn generate_extract(self) -> TokenStream {
        let Self { expr, nested, .. } = self;
        let data = Ident::new("data", Span::mixed_site());
        let err = data.to_string() + " is null";
        let (pats, tokens): (Vec<_>, Vec<_>) =
            nested.generate_extract(data.clone(), data.to_string());
        quote! {
            let #data = ( #expr ).ok_or(#err)?;
            let ( #(#pats),* ) = {
                #(#tokens)*
                ( #(#pats),* )
            };
        }
    }
}

impl Node {
    fn generate_extract(self, data: Ident, path: String) -> (syn::Pat, TokenStream) {
        let Self {
            ident,
            optional,
            iterable,
            nested,
        } = self;

        let path = path + " -> " + ident.to_string().as_str();

        let assign = if optional {
            let err = path.clone() + " is null";
            quote!(let #ident = #data.#ident.ok_or(#err)?;)
        } else {
            quote!(let #ident = #data.#ident;)
        };

        let Some(inner) = nested else {
            return (parse_quote!(#ident), assign);
        };

        let (pats, tokens) = inner.generate_extract(ident.clone(), path);
        let (pat, tokens_);
        // TODO: consider
        // - verifying that no nested `[]` exist
        // - detecting any `?` in the subtree and setting the return type accordingly
        if iterable {
            pat = parse_quote!(#ident);
            tokens_ = quote! {
                #assign
                let #ident = #ident.into_iter().map(|#ident| -> Result<_, &'static str> {
                    #(#tokens)*
                    Ok(( #(#pats),* ))
                });
            };
        } else {
            pat = parse_quote!( (#(#pats),*) );
            tokens_ = quote! {
                #assign
                let ( #(#pats),* ) = {
                    #(#tokens)*
                    ( #(#pats),* )
                };
            };
        }
        (pat, tokens_)
    }
}

impl Nested {
    fn generate_extract(self, data: Ident, path: String) -> (Vec<syn::Pat>, Vec<TokenStream>) {
        match self {
            Self::Nodes(nodes) => nodes
                .into_iter()
                .map(|n| n.generate_extract(data.clone(), path.clone()))
                .unzip(),
            Self::Variant(Variant { path: var, nodes }) => {
                let path = path + " ... on " + var.to_token_stream().to_string().as_str();
                let err = path.clone() + " is null";
                let val = Ident::new("val", Span::mixed_site());
                let assign = quote! {
                    let #var(#val) = #data else {
                        return Err(#err);
                    };
                };

                let mut tokens_ = vec![assign];
                let (pats, tokens): (Vec<_>, Vec<_>) = nodes
                    .into_iter()
                    .map(|n| n.generate_extract(val.clone(), path.clone()))
                    .unzip();
                tokens_.extend(tokens);
                (pats, tokens_)
            }
        }
    }
}
