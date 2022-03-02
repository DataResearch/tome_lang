use std::iter::{Peekable, Iterator};
use crate::token::Token;

pub enum LexicalError {
    UnexpectedSymbol(char),
    UnexpectedEndOfStream(),

}


pub trait Lexer {
    fn lexing<I> (iterator: &mut Peekable<I>) -> Result<Token, LexicalError> 
        where I: Iterator<Item=char>;
}
