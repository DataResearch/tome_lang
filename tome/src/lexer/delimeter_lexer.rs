use std::iter::{Peekable, Iterator};
use crate::token::Token;
use crate::predicate;
use super::lexer::{self, LexicalError};

pub struct DelimeterLexer{}

impl lexer::Lexer for DelimeterLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let peek = iterator.next();

        if let Some(character) = peek {
            
            if predicate::is_delimeter(&character) {
                return Ok(Token::Delimiter(character));
            }
            else {
                return Err(LexicalError::UnexpectedSymbol(character))
            }
        }
        else {
            Err(LexicalError::UnexpectedEndOfStream())
        }
    }
}
