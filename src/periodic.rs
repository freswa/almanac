use std::fmt;
use std::str::FromStr;
use std::collections::HashSet;

use chrono::{Duration, Weekday};

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
    pub byday: Option<HashSet<Weekday>>,
    pub wkst: Weekday,
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
            byday: None,
            wkst: Weekday::Mon,
        }
    }

    pub fn set_param(&mut self, param: &str, value: &str) -> Result<(), EventError> {
        match param {
            "FREQ" => self.freq = value.parse()?,
            "INTERVAL" => self.interval = value.parse()?,
            "COUNT" => self.count = Some(value.parse()?),
            "UNTIL" => self.until = Some(Date::parse(&value, "")?),
            "BYDAY" => self.byday = Some(parse_byday(value)?),
            "WKST" => self.wkst = parse_weekday(value)?,
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

        let duration = self.next_duration(self.start);
        self.start = self.start + duration;
        self.end = self.end + duration;
        self.count += 1;

        Some(event)
    }
}

impl<'a> Iter<'a> {
    fn next_duration(&self, date: Date) -> Duration {
        let p = self.periodic;
        match p.freq {
            Freq::Secondly => Duration::seconds(p.interval),
            Freq::Minutely => Duration::minutes(p.interval),
            Freq::Hourly => Duration::hours(p.interval),
            Freq::Daily => Duration::days(p.interval),
            Freq::Weekly => {
                match &p.byday {
                    None => Duration::weeks(p.interval),
                    Some(byday) => {
                        let mut weekday = date.weekday().succ();
                        let mut days = 1;
                        if weekday == p.wkst {
                            days += 7 * (p.interval - 1);
                        }
                        while !byday.contains(&weekday) {
                            weekday = weekday.succ();
                            days += 1;
                            if weekday == p.wkst {
                                days += 7 * (p.interval - 1);
                            }
                        }
                        Duration::days(days)
                    }
                }
            }
            Freq::Monthly => {
                let new_date = if date.month() == 12 {
                    date.with_month(1)
                        .unwrap()
                        .with_year(date.year() + 1)
                        .unwrap()
                } else {
                    date.with_month(date.month() + 1).unwrap()
                };
                new_date - date
            }
            Freq::Yearly => date.with_year(date.year() + 1).unwrap() - date,
        }
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

fn parse_byday(s: &str) -> Result<HashSet<Weekday>, EventError> {
    let mut byday = HashSet::new();
    for v in s.split(",") {
        byday.insert(parse_weekday(v)?);
    }
    Ok(byday)
}

fn parse_weekday(s: &str) -> Result<Weekday, EventError> {
    match s {
        "MO" => Ok(Weekday::Mon),
        "TU" => Ok(Weekday::Tue),
        "WE" => Ok(Weekday::Wed),
        "TH" => Ok(Weekday::Thu),
        "FR" => Ok(Weekday::Fri),
        "SA" => Ok(Weekday::Sat),
        "SU" => Ok(Weekday::Sun),
        _ => Err(EventError::BydayError),
    }
}
