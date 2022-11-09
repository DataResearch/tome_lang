use std::iter::{Peekable, Iterator};
use crate::token::token::{Bracket, Token};
use crate::token::predicate;
use super::lexer::{Lexer, LexicalError};

pub struct BracketLexer{}

impl Lexer for BracketLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        match iterator.next() {
            Some(key) if predicate::is_bracket(&key) => {
                let conversion = Bracket::try_from(&key);
                match conversion {
                    Ok(bracketType) => Ok(Token::Bracket(bracketType)),
                    Err(_) => Err(LexicalError::UnexpectedSymbol(key))
                }
            },
            _ => Err(LexicalError::UnexpectedEndOfStream)
        }
    }
}
