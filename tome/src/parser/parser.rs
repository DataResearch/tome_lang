
pub enum ParserError {
    UnknownToken,
    UnexpectedToken,
    UnexpectedEndOfTokenStream,
    InternalLookupError // for when a lookup table internally returns no result
}

pub trait Parser {

}