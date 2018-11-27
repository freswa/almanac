use std::cmp::{Ordering, Ord};
use std::fmt;
use std::str::FromStr;

use chrono::Duration;

use date::Date;
use errors::EventError;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct Event {
    pub start: Date,
    pub end: End,
    pub summary: String,
    pub location: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub enum Status {
    Confirmed,
    Tentative,
    Canceled,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub enum End {
    Date(Date),
    Duration(Duration),
}


impl Event {
    pub fn new() -> Event {
        return Event {
            summary: "".to_string(),
            location: "".to_string(),
            description: "".to_string(),
            status: Status::Confirmed,
            start: Date::new(),
            end: End::Date(Date::new()),
        };
    }

    pub fn end_date(&self) -> Date {
        match self.end {
            End::Date(date) => date,
            End::Duration(duration) => self.start + duration,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}-{:?}: {}",
            self.start,
            self.end_date(),
            self.summary
        )?;
        if !self.location.is_empty() {
            write!(f, " ({})", self.location)?;
        }
        if !self.description.is_empty() {
            write!(f, "\n\t{}", self.description)?;
        }
        Ok(())
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.start.cmp(&other.start);
        if ord == Ordering::Equal {
            self.end_date().cmp(&other.end_date())
        } else {
            ord
        }
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


pub fn get<'a>(events: &'a Vec<Event>, first: &Date, last: &Date) -> &'a [Event] {
    match events.iter().position(|ref e| e.start >= *first) {
        None => &[],
        Some(i) => {
            let j = events
                .iter()
                .position(|ref e| e.end_date() > *last)
                .unwrap_or(events.len());
            &events[i..j]
        }
    }
}
