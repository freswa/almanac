use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

use ical::parser;
use ical::IcalParser;
use chrono;
use chrono_tz::{Tz, UTC};
use chrono::prelude::{DateTime, TimeZone};


#[derive(Debug)]
pub enum Date {
    Time(DateTime<Tz>),
    AllDay(chrono::Date<Tz>),
}

#[derive(Debug)]
pub struct Event {
    pub start: Date,
    pub end: Date,
    pub summary: String,
    pub location: String,
    pub description: String,
}

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
                    "DTSTART" => event.start = parse_date(value, time_zone)?,
                    "DTEND" => event.end = parse_date(value, time_zone)?,
                    _ => (),
                };
            }
            events.push(event);
        }
    }

    Ok(events)
}

fn parse_date(date: String, time_zone: String) -> Result<Date, IcsError> {
    let tz: Tz = time_zone.parse().unwrap_or(UTC);
    let date = match date.find("T") {
        Some(_) => {
            let time = tz.datetime_from_str(&date, "%Y%m%dT%H%M%S").unwrap_or(
                UTC.timestamp(
                    0,
                    0,
                ),
            );
            Date::Time(time)
        }
        None => {
            Date::AllDay(tz.ymd(
                i32::from_str(&date[0..4])?,
                u32::from_str(&date[4..6])?,
                u32::from_str(&date[6..8])?,
            ))
        }
    };
    Ok(date)
}

impl Event {
    fn new() -> Event {
        return Event {
            summary: "".to_string(),
            location: "".to_string(),
            description: "".to_string(),
            start: Date::Time(UTC.timestamp(0, 0)),
            end: Date::Time(UTC.timestamp(0, 0)),
        };
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.start, self.summary)?;
        if !self.location.is_empty() {
            write!(f, " ({})", self.location)?;
        }
        if !self.description.is_empty() {
            write!(f, "\n\t{}", self.description)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum IcsError {
    IoError(io::Error),
    IcalError(parser::errors::Error),
    IntError(ParseIntError),
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
