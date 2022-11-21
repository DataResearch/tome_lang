use std::iter::{Peekable, Iterator};
use crate::extensions::iter::BlockingIter;
use crate::token::token::Token;
use crate::token::predicate;
use super::lexer::{Lexer, LexicalError};

pub struct NumericLexer{}

impl Lexer for NumericLexer {
    // TODO (@CodingChris): we should handle numeric literatals for octal, hex, and binary
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let peek = iterator.peek().map_or(None, |x| Some(*x));
        let numericBlob = iterator.blocking_take(predicate::is_numeric);

        if numericBlob.is_empty() {
            return match peek {
                Some(value) => Err(LexicalError::UnexpectedSymbol(value)),
                _ => Err(LexicalError::UnexpectedEndOfStream)
            };
        }

        Ok(Token::Numeric(String::from_iter(numericBlob)))
    }
}
