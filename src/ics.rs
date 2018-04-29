use std::io;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use std::cmp::Ordering;

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
pub enum Status {
    Confirmed,
    Tentative,
    Canceled,
}

#[derive(Debug)]
pub struct Event {
    pub start: Date,
    pub end: Date,
    pub summary: String,
    pub location: String,
    pub description: String,
    pub status: Status,
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
                    "STATUS" => event.status = Status::from_str(&value)?,
                    "DTSTART" => event.start = parse_date(value, time_zone)?,
                    "DTEND" => event.end = parse_date(value, time_zone)?,
                    _ => (),
                };
            }
            events.push(event);
        }
    }

    events.sort_by(|a, b| a.start.cmp(&b.start));
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
            status: Status::Confirmed,
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

impl Date {
    fn cmp(&self, other: &Date) -> Ordering {
        match *self {
            Date::Time(t1) => {
                match *other {
                    Date::Time(t2) => t1.cmp(&t2),
                    Date::AllDay(d) => cmp_date_time(&d, &t1).reverse(),
                }
            }
            Date::AllDay(d1) => {
                match *other {
                    Date::Time(t) => cmp_date_time(&d1, &t),
                    Date::AllDay(d2) => d1.cmp(&d2),
                }
            }
        }
    }
}

impl FromStr for Status {
    type Err = IcsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONFIRMED" => Ok(Status::Confirmed),
            "TENTATIVE" => Ok(Status::Tentative),
            "CANCELED" => Ok(Status::Canceled),
            _ => Err(IcsError::StatusError),
        }
    }
}

fn cmp_date_time<T: TimeZone>(date: &chrono::Date<T>, time: &DateTime<T>) -> Ordering {
    let d2 = time.date();
    if date.eq(&d2) {
        return Ordering::Less;
    }
    date.cmp(&d2)
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
