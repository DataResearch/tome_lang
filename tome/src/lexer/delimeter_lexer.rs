use std::iter::{Peekable, Iterator};
use crate::token::token::{Delimeter, Token};
use crate::token::predicate;
use super::lexer::{Lexer, LexicalError};

pub struct DelimeterLexer{}

impl Lexer for DelimeterLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        match iterator.next() {
            Some(key) if predicate::is_bracket(&key) => {
                let conversion = Delimeter::try_from(&key);
                match conversion {
                    Ok(delimeterType) => Ok(Token::Delimiter(delimeterType)),
                    Err(_) => Err(LexicalError::UnexpectedSymbol(key))
                }
            },
            _ => Err(LexicalError::UnexpectedEndOfStream)
        }
    }
}
