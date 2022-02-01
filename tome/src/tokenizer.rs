use crate::token::TokenStream;

pub struct Tokenizer {
    code: Vec<char>,
    marker: usize
}

enum ExhaustionState {
    Open,
    Exhausted
}

impl Tokenizer {

    fn reset(&mut self) {
        self.code = vec![];
        self.marker = 0usize;
    }

    fn current(&self) -> char {
        self.code[self.marker]
    }

    fn peek(&self) -> Option<char> {
        self.code.get(self.marker + 1).map_or(None, |x| Some(*x))
    }

    fn next(&mut self) -> ExhaustionState {
        self.marker += 1;
        if self.marker == self.code.len() { ExhaustionState::Exhausted } else { ExhaustionState::Open }
    }
}

impl Tokenizer {

    pub fn new() -> Tokenizer {
        Tokenizer{
            code: vec![],
            marker: 0usize
        }
    }



    pub fn tokenize(&mut self, contents: &str) -> TokenStream {
        // reset internal Tokenzier state
        self.reset();

        // load content into the Tokenizer
        let blob = contents.to_owned();
        self.code = blob.chars().collect();

        

        vec!{}
    }

}
