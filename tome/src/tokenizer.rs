use crate::lexer::lexer::Lexer;
use crate::token::{Token, OPERATORS, DELIMETERS, KEYWORDS, BRACKETS};
use crate::iter::BlockingIter;
use crate::predicate;
use crate::lexer::numeric_lexer::NumericLexer;

use std::iter::{Iterator, Peekable};

pub struct LexicalTokenIterator<T: Iterator<Item=char>> {
    code: Peekable<T>
}

impl<T> LexicalTokenIterator<T> 
    where T: Iterator<Item=char> {

    pub fn new(value: T) -> LexicalTokenIterator<T> {
        LexicalTokenIterator{
            code: value.peekable()
        }
    }

}

impl<T> Iterator for LexicalTokenIterator<T> 
    where T: Iterator<Item=char> {

    type Item = Token;

    // Produces a single new token from the internals of the Tokenzier
    // TODO (@CodingChris): This function can definetly be split up in sub functions / methods.
    // I don't like the return Some(Token::<Type>(<Value>)) stuff. Can be better when split up in smaller units.
    // This might then also cleanup some bugs in parsing currently marked with "TODO".
    fn next(&mut self) -> Option<Self::Item> {

        // trim whitespaces
        self.code.blocking_skip(predicate::is_whitespace);

        // check if there is something to evaluate
        // TODO (@Christian): Code is borrowed mutably here - which requires char to be used
        // as a copy type instead of a reference. The usages of & in the predicates look weired now.
        if let Some(&x) = self.code.peek() {

            // Parse numeric token
            if predicate::is_numeric(&x) {

                if let Ok(token) = NumericLexer::lexing(&mut self.code) {
                    return Some(token);
                }
            }

            // test for string literals
            if x == '"' {
            
                // take everything until we find a " codepoint. This is the end!
                // TODO (@CodingChris): make it better - it hurts my eyes
                // also: test if >> " this is part of a li EOF<< creates issues or crashes..
                self.code.next(); // cut of " codepoint
                let string_literal = self.code.blocking_take(|x| x != &'"');
                self.code.next(); // cut of " codepoint

                let string: String = string_literal.into_iter().collect();
                return Some(Token::StringLiteral(string));

            }

            // test for operators, brackets, seperators, dots, ...
            if predicate::is_symbol(&x) {
            
                let symbol_list = self.code.blocking_take(predicate::is_symbol);
                let symbol: String = symbol_list.into_iter().collect();

                // test for the very unique dot operator
                if symbol == "." {
                    return Some(Token::Dot);
                }
                // TODO (@CodingChris): operator much? - clean this up
                if OPERATORS.contains(&symbol.as_ref()) {
                    return Some(Token::Operator(symbol));
                }
                if BRACKETS.contains(&symbol.as_ref()) {
                    return Some(Token::Bracket(symbol));
                }
                if DELIMETERS.contains(&symbol.as_ref()) {
                    return Some(Token::Delimiter(symbol));
                }
                
                return Some(Token::Unknown);
            }

            // test for keywords and literals (literals have their own predicate)
            if predicate::is_alpha(&x) {
            
                let keying_list = self.code.blocking_take(|x| predicate::is_alphanumeric(x) || x == &'_');
                let words: String = keying_list.into_iter().collect();

                if KEYWORDS.contains(&words.as_ref()) {
                    return Some(Token::Keyword(words));
                }
                else {
                    return Some(Token::Identifier(words));
                }

            }
            
            // no parsing case matched - resulting in an unknown token
            self.code.next(); // skip a single token - we might find something useful in the next round over
            return Some(Token::Unknown)

        }
        else {
            return None;
        }

    }

}
