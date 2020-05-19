use std::io;
use std::num::ParseIntError;
use ical::parser;
use toml;

#[derive(Debug)]
pub enum EventError {
    IcalError(parser::errors::Error),
    IntError(ParseIntError),
    StatusError,
    FreqError,
    BydayError,
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

#[derive(Debug)]
pub enum ConfigError {
    IOError(io::Error),
    ParseError(toml::de::Error),
    MissingPath,
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::IOError(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> ConfigError {
        ConfigError::ParseError(err)
    }
}
