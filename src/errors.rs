use std::num::ParseIntError;
use ical::parser;

#[derive(Debug)]
pub enum EventError {
    IcalError(parser::errors::Error),
    IntError(ParseIntError),
    StatusError,
}

impl From<parser::errors::Error> for EventError {
    fn from(err: parser::errors::Error) -> EventError {
        EventError::IcalError(err)
    }
}

impl From<ParseIntError> for EventError {
    fn from(err: ParseIntError) -> EventError {
        EventError::IntError(err)
    }
}
