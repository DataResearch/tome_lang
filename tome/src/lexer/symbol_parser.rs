use std::iter::{Peekable, Iterator};
use crate::iter::{BlockingIter, self};
use crate::token::Token;
use crate::predicate;

// Note (@CodingChris): We could for these parsers to take all symbols they can.
// but it is simpler to just let them take a single symbol of the iterator stream.
// And the main loop will just rerun it for the next token. This simplifies the 
// code a whole lot and reduces errors in the long run. Also the handler code for the
// results will be the same in all cases!

// Parses a set of brakets into a list of Tokens
fn parse_brackets<I> (iterator: &mut Peekable<I>) -> Token where I: Iterator<Item=char> {
    if !predicate::is_bracket(iterator.peek().unwrap()) {
        panic!("Next symbolic token isn't a bracket when asked to create a bracket token.");
    }

    Token::Bracket(String::from(iterator.next().unwrap()))
}
