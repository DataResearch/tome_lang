use std::iter::{Peekable, Iterator};
use crate::extensions::iter::BlockingIter;
use crate::token::{predicate, token::Token, token::OPERATORS};
use super::lexer::{Lexer, LexicalError};

pub struct OperatorLexer{}

impl Lexer for OperatorLexer {
    
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {
        let peek = iterator.peek().map_or(None, |x| Some(*x));
        let operatorBlob = iterator.blocking_take(predicate::is_symbol);

        if operatorBlob.is_empty() {
            return match peek {
                Some(value) => Err(LexicalError::UnexpectedSymbol(value)),
                _ => Err(LexicalError::UnexpectedEndOfStream)
            };
        }

        let operatorCandidate: String = operatorBlob.into_iter().collect();
        if OPERATORS.contains(&(&operatorCandidate as &str)) {
            return Ok(Token::Operator(operatorCandidate));
        }
        else {
            return Err(LexicalError::UnexpectedSymbolSequence(operatorCandidate));
        }
    }
}
