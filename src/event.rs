use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use chrono;
use chrono::TimeZone;
use chrono_tz::{Tz, UTC};

use errors::EventError;


#[derive(Debug)]
pub struct Event {
    pub start: Date,
    pub end: Date,
    pub summary: String,
    pub location: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug)]
pub enum Date {
    Time(chrono::DateTime<Tz>),
    AllDay(chrono::Date<Tz>),
}

#[derive(Debug)]
pub enum Status {
    Confirmed,
    Tentative,
    Canceled,
}


impl Event {
    pub fn new() -> Event {
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
    pub fn cmp(&self, other: &Self) -> Ordering {
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

    pub fn parse(date: &String, time_zone: &String) -> Result<Self, EventError> {
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
}

impl FromStr for Status {
    type Err = EventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONFIRMED" => Ok(Status::Confirmed),
            "TENTATIVE" => Ok(Status::Tentative),
            "CANCELED" => Ok(Status::Canceled),
            _ => Err(EventError::StatusError),
        }
    }
}

fn cmp_date_time<T: TimeZone>(date: &chrono::Date<T>, time: &chrono::DateTime<T>) -> Ordering {
    let d2 = time.date();
    if date.eq(&d2) {
        return Ordering::Less;
    }
    date.cmp(&d2)
}
