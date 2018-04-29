use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::num::ParseIntError;

use ical::parser;
use ical::IcalParser;

use event::{Event, Date};


pub fn parse<P: AsRef<Path>>(ics: P) -> Result<Vec<Event>, IcsError> {
    let buf = BufReader::new(File::open(ics)?);
    let reader = IcalParser::new(buf);
    let mut events = Vec::new();

    for line in reader {
        for ev in line?.events {
            let mut event = Event::new();
            for property in ev.properties {
                let value = property.value.unwrap_or("".to_string());
                let mut time_zone = "".to_string();

                for (param, value) in property.params.unwrap_or(vec![]) {
                    if param == "TZID" && value.len() > 0 {
                        time_zone = value[0].clone();
                    }
                }

                match property.name.as_ref() {
                    "SUMMARY" => event.summary = value,
                    "LOCATION" => event.location = value,
                    "DESCRIPTION" => event.description = value,
                    "STATUS" => event.status = value.parse()?,
                    "DTSTART" => event.start = Date::parse(value, time_zone)?,
                    "DTEND" => event.end = Date::parse(value, time_zone)?,
                    _ => (),
                };
            }
            events.push(event);
        }
    }

    events.sort_by(|a, b| a.start.cmp(&b.start));
    Ok(events)
}

#[derive(Debug)]
pub enum IcsError {
    IoError(io::Error),
    IcalError(parser::errors::Error),
    IntError(ParseIntError),
    StatusError,
}

impl From<io::Error> for IcsError {
    fn from(err: io::Error) -> IcsError {
        IcsError::IoError(err)
    }
}

impl From<parser::errors::Error> for IcsError {
    fn from(err: parser::errors::Error) -> IcsError {
        IcsError::IcalError(err)
    }
}

impl From<ParseIntError> for IcsError {
    fn from(err: ParseIntError) -> IcsError {
        IcsError::IntError(err)
    }
}
