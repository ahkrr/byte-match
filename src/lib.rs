#![warn(clippy::all, clippy::pedantic)]

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::borrow::Borrow;
use std::fmt::Display;
use std::iter::FromIterator;
use std::str::FromStr;

#[proc_macro]
pub fn bu8(input: TokenStream) -> TokenStream {
    match bu8_impl(input) {
        Ok(bytefield) => bytefield,
        Err(error) => error.into_compile_error(),
    }
}

fn bu8_impl(input: TokenStream) -> Result<TokenStream, SyntaxError> {
    let iter = input.into_iter().peekable();

    let mut output = String::from('[');
    let mut dot_count = 0;
    let mut too_many_idents = false;
    for it in iter {
        match &it {
            TokenTree::Literal(lit) => match lit.to_string().as_bytes() {
                [b'"', x @ .., b'"']
                | [b'b', b'"', x @ .., b'"']
                | [b'\'', x @ .., b'\'']
                | [b'b', b'\'', x @ .., b'\''] => {
                    for it in x {
                        output.push_str("b'");
                        output.push(*it as char);
                        output.push_str("', ");
                    }
                }
                _ => {
                    return Err(syntax(
                        it,
                        r#"only "lit" | b"lit" | 'l' | b'l' literals allowed "#,
                    ));
                }
            },
            TokenTree::Ident(idt) => {
                if too_many_idents {
                    return Err(syntax(it, "only one ident allowed"));
                } else {
                    output.push_str(&idt.to_string());
                    too_many_idents = true;
                }
            }
            TokenTree::Punct(pu) => {
                match pu.to_string().as_ref() {
                    "." => match dot_count {
                        0 => {
                            output.push('.');
                            dot_count += 1;
                        }
                        1 => {
                            output.push_str(". , ");
                            dot_count += 1;
                        }
                        _ => {
                            return Err(syntax(it, "only two dots allowd .."));
                        }
                    },
                    "@" => output.push_str(" @ "),
                    _ => {
                        return Err(syntax(it, "illegal punctuation"));
                    }
                };
            }
            _ => {
                return Err(syntax(it, "illegal token"));
            }
        }
    }
    output.push(']');
    let token_out = TokenStream::from_str(&output).unwrap();
    Ok(token_out)
}

pub(crate) struct SyntaxError {
    message: String,
    span: Span,
}

impl SyntaxError {
    pub(crate) fn into_compile_error(self) -> TokenStream {
        // compile_error! { $message }
        TokenStream::from_iter(vec![
            TokenTree::Ident(Ident::new("compile_error", self.span)),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(self.span);
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter(vec![TokenTree::Literal({
                        let mut string = Literal::string(&self.message);
                        string.set_span(self.span);
                        string
                    })])
                });
                group.set_span(self.span);
                group
            }),
        ])
    }
}

fn syntax<T: Borrow<TokenTree>, M: Display>(token: T, message: M) -> SyntaxError {
    SyntaxError {
        message: message.to_string(),
        span: token.borrow().span(),
    }
}
