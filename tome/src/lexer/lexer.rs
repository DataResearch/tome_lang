use std::iter::{Peekable, Iterator};
use crate::token::token::Token;

#[allow(clippy::enum_variant_names)]
pub enum LexicalError {
    UnexpectedSymbol(char),
    UnexpectedSymbolSequence(String),
    UnexpectedEndOfStream,
}

pub trait Lexer {
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char>;
}
