use std::iter::{Peekable, Iterator};
use crate::iter::BlockingIter;
use crate::token::Token;
use crate::predicate;
use super::lexer::{self, LexicalError};

pub struct NumericLexer{}

impl lexer::Lexer for NumericLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let peek = iterator.peek();

        if let Some(character) = peek {

            let chars = iterator.blocking_take(predicate::is_numeric);

            if !chars.is_empty() {
                Ok(Token::Numeric(chars.into_iter().collect()))
            }
            else {
                Err(LexicalError::UnexpectedSymbol(*character))
            }
        }
        else {
            Err(LexicalError::UnexpectedEndOfStream())
        }
    }
}
