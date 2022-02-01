
/// Represents a single file for the compiler.
pub struct LoadedFile {
    lines: Vec<String>,
    _location: String
}

fn read_lines(filename: &str) -> Vec<String> {

    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    // TODO (@CodingChris): clean this up to avoid panics
    let file = File::open(filename).unwrap();
    BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
}

impl LoadedFile {

    pub fn new(location: &str) -> LoadedFile {
        LoadedFile{
            lines: read_lines(location),
            _location: location.to_owned()
        }
    }

    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
}
