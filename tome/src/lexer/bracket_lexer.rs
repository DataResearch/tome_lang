use std::iter::{Peekable, Iterator};
use crate::token::Token;
use crate::predicate;

use super::lexer::{Lexer, LexicalError};

pub struct BracketLexer{}

impl Lexer for BracketLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let value = iterator.next();

        if let Some(character) = value {
            
            if predicate::is_bracket(&character) {
                Ok(Token::Bracket(character))
            }
            else {
                Err(LexicalError::UnexpectedSymbol(character))
            }

        }
        else {
            Err(LexicalError::UnexpectedEndOfStream())
        }

    }
}
