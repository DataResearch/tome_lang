use std::iter::{Peekable, Iterator};
use crate::iter::BlockingIter;
use crate::token::Token;
use crate::predicate;
use super::lexer::{self, LexicalError};

struct NumericLexer{}

impl lexer::Lexer for NumericLexer {
    
    fn parse_numeric_literal<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char> 
    {

        let chars = iterator.blocking_take(predicate::is_numeric);

        if !chars.is_empty() {
            Ok(Token::Numeric(chars.into_iter().collect()))
        }
        else {
            Err(LexicalError::UnexpectedSymbol(iterator.peek().unwrap_or(&" ")))
        }

    }
}
