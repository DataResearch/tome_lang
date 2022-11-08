use std::iter::{Peekable, Iterator};
use crate::{iter::BlockingIter, token::OPERATORS};
use crate::token::Token;
use crate::predicate;
use super::lexer::{self, LexicalError};

pub struct OperatorLexer{}

impl lexer::Lexer for OperatorLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let peek = iterator.peek();

        if let Some(_) = peek {

            let chars = iterator.blocking_take(predicate::is_symbol);
            let symbols: String = chars.into_iter().collect();

            if OPERATORS.contains(&(&symbols as &str)) {
                return Ok(Token::Operator(symbols));
            }
            else {
                return Err(LexicalError::UnexpectedSymbolSequence(symbols));
            }
        }
        else {
            Err(LexicalError::UnexpectedEndOfStream())
        }
    }
}
