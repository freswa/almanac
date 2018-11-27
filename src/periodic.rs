use std::fmt;
use std::str::FromStr;

use chrono::Duration;

use date::Date;
use event::{Event, End};
use errors::EventError;

#[derive(Debug)]
pub struct Periodic {
    pub event: Event,
    pub freq: Freq,
    pub interval: i64,
    pub count: Option<i64>,
    pub until: Option<Date>,
}

#[derive(Debug)]
pub enum Freq {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Periodic {
    pub fn new() -> Self {
        Self {
            event: Event::new(),
            freq: Freq::Secondly,
            interval: 1,
            count: None,
            until: None,
        }
    }

    pub fn set_param(&mut self, param: &str, value: &str) -> Result<(), EventError> {
        match param {
            "FREQ" => self.freq = value.parse()?,
            "INTERVAL" => self.interval = value.parse()?,
            "COUNT" => self.count = Some(value.parse()?),
            "UNTIL" => self.until = Some(Date::parse(&value, "")?),
            _ => (),
        }
        Ok(())
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            periodic: self,
            start: self.event.start,
            end: self.event.end_date(),
            count: 0,
        }
    }
}

pub struct Iter<'a> {
    periodic: &'a Periodic,
    start: Date,
    end: Date,
    count: i64,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.periodic;

        if (p.until.is_some() && self.start <= p.until.unwrap()) ||
            (p.count.is_some() && self.count >= p.count.unwrap())
        {
            return None;
        }

        let mut event = p.event.clone();
        event.start = self.start;
        event.end = End::Date(self.end);

        self.count += 1;
        self.start = p.freq.next_date(self.start, p.interval);
        self.end = p.freq.next_date(self.end, p.interval);

        Some(event)
    }
}

impl fmt::Display for Periodic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.freq)?;
        if self.interval != 1 {
            write!(f, "({})", self.interval)?;
        }
        write!(f, ": {}", self.event)?;
        Ok(())
    }
}

impl Freq {
    pub fn next_date(&self, date: Date, count: i64) -> Date {
        match self {
            Freq::Secondly => date + Duration::seconds(count),
            Freq::Minutely => date + Duration::minutes(count),
            Freq::Hourly => date + Duration::hours(count),
            Freq::Daily => date + Duration::days(count),
            Freq::Weekly => date + Duration::weeks(count),
            Freq::Monthly => {
                let month = date.month();
                if month == 12 {
                    let date = date.with_month(1).unwrap();
                    date.with_year(date.year() + 1).unwrap()
                } else {
                    date.with_month(month + 1).unwrap()
                }
            }
            Freq::Yearly => date.with_year(date.year() + 1).unwrap(),
        }
    }
}

impl FromStr for Freq {
    type Err = EventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SECONDLY" => Ok(Freq::Secondly),
            "MINUTELY" => Ok(Freq::Minutely),
            "HOURLY" => Ok(Freq::Hourly),
            "DAILY" => Ok(Freq::Daily),
            "WEEKLY" => Ok(Freq::Weekly),
            "MONTHLY" => Ok(Freq::Monthly),
            "YEARLY" => Ok(Freq::Yearly),
            _ => Err(EventError::FreqError),
        }
    }
}
