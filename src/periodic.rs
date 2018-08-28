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
    pub count: i64,
    pub until: Date,
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
            until: Date::empty(),
            count: 0,
        }
    }

    pub fn set_param(&mut self, param: &str, value: &str) -> Result<(), EventError> {
        match param {
            "FREQ" => self.freq = value.parse()?,
            "INTERVAL" => self.interval = value.parse()?,
            "COUNT" => self.count = value.parse()?,
            "UNTIL" => self.until = Date::parse(&value, "")?,
            _ => (),
        }
        Ok(())
    }

    pub fn get(&self, first: &Date, last: &Date) -> Vec<Event> {
        let mut start = self.event.start;
        let mut end = self.event.end_date();
        let mut events = Vec::new();
        let mut count = 0;
        while start <= *last {
            if (!self.until.is_empty() && start <= self.until) ||
                (count != 0 && count >= self.count)
            {
                break;
            }

            if start >= *first {
                let mut e = self.event.clone();
                e.start = start;
                e.end = End::Date(end);
                events.push(e);
                count += count;
            }
            start = start + self.freq.duration(self.interval);
            end = end + self.freq.duration(self.interval);
        }
        events
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
    pub fn duration(&self, count: i64) -> Duration {
        match self {
            Freq::Secondly => Duration::seconds(count),
            Freq::Minutely => Duration::minutes(count),
            Freq::Hourly => Duration::hours(count),
            Freq::Daily => Duration::days(count),
            Freq::Weekly => Duration::weeks(count),
            Freq::Monthly => Duration::days(count * 30), // FIXME
            Freq::Yearly => Duration::days(count * 365), // FIXME
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
