#![allow(dead_code)]

use tome::{self, token::TokenStream};

const PREANALYTICAL: u32 = 0b00000001;
const LEXICAL: u32 = 0b00000010;
const ABSTRACT_SYNTAX_TREE: u32 = 0b00000100;
const EXECUTION_STATE: u32 = 0b00001000;
const TYPE_TABLE: u32 = 0b00010000;

struct VerboseOutputOptions {
    flags: u32
}

impl VerboseOutputOptions {

    fn new(flag: u32) -> VerboseOutputOptions {
        VerboseOutputOptions{ flags: flag }
    }

    fn is_pre_analytical(&self) -> bool {
        (self.flags & &PREANALYTICAL) != 0
    }

    fn is_lexical(&self) -> bool {
        (self.flags & &LEXICAL) != 0
    }

    fn is_ast(&self) -> bool {
        (self.flags & &ABSTRACT_SYNTAX_TREE) != 0
    }

    fn is_execution_state(&self) -> bool {
        (self.flags & &EXECUTION_STATE) != 0
    }

    fn is_typetable(&self) -> bool {
        (self.flags & &TYPE_TABLE) != 0
    }
}

struct Pipeline {
    option: VerboseOutputOptions
}

impl Pipeline {

    fn new(flags: VerboseOutputOptions) -> Pipeline {
        Pipeline { 
            option: flags
        }
    }

    fn execute(&mut self, code: &str) {

        if self.option.is_pre_analytical() {
            println!("{:?}", code);
        }

        let lexer = tome::tokenizer::LexicalTokenIterator::new(code.chars());
        let tokens: TokenStream = lexer.collect();

        if self.option.is_lexical() {
            println!("{:?}", tokens);
        }
        
    }

}

fn main() {

    use std::io::Write;

    let mut executor = Pipeline::new(VerboseOutputOptions::new(PREANALYTICAL | LEXICAL));
    let mut userinput = String::new();
    
    while !userinput.starts_with("quit();") {
        // print user input markers
        print!(">> ");
        std::io::stdout().flush().unwrap(); // line buffered - flush buffer

        // read user input from the console
        userinput.clear();
        std::io::stdin()
            .read_line(&mut userinput)
            .expect("Should never have failed in the firstplace. Report: @CodingChris");

        executor.execute(&userinput);
    }

}
