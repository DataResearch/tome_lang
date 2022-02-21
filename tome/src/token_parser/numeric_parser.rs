use std::iter::{Peekable, Iterator};
use crate::iter::BlockingIter;
use crate::token::Token;
use crate::predicate;

/// Parse all numbers as numeric tokens
fn parse_numeric_literal<I> (iterator: &mut Peekable<I>) -> Token where I: Iterator<Item=char> {
    let chars = iterator.blocking_take(predicate::is_numeric);
    Token::Numeric(chars.into_iter().collect())
}
