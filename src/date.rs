use std::cmp::{Ordering, Ord, PartialEq, PartialOrd};
use std::str::FromStr;

use errors::EventError;

use chrono;
use chrono::TimeZone;
use chrono_tz::{Tz, UTC};


#[derive(Debug, Copy, Clone, Eq)]
pub enum Date {
    Time(chrono::DateTime<Tz>),
    AllDay(chrono::Date<Tz>),
}


impl Date {
    pub fn empty() -> Date {
        Date::Time(UTC.timestamp(0, 0))
    }

    pub fn parse(date: &String, time_zone: &String) -> Result<Self, EventError> {
        let absolute_time = date.chars().rev().next().unwrap() == 'Z';
        let tz: Tz = if absolute_time {
            UTC
        } else {
            // FIXME: this should not be UTC but local timezone
            time_zone.parse().unwrap_or(UTC)
        };

        let date = match date.find("T") {
            Some(_) => {
                let date_pattern = if absolute_time {
                    "%Y%m%dT%H%M%SZ"
                } else {
                    "%Y%m%dT%H%M%S"
                };
                let time = tz.datetime_from_str(&date, date_pattern).unwrap_or(
                    UTC.timestamp(0, 0),
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

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
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

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn cmp_date_time<T: TimeZone>(date: &chrono::Date<T>, time: &chrono::DateTime<T>) -> Ordering {
    let d2 = time.date();
    if date.eq(&d2) {
        return Ordering::Less;
    }
    date.cmp(&d2)
}

#[cfg(test)]
mod tests {
    use super::Date;
    use chrono::Datelike;
    use chrono::Timelike;

    #[test]
    fn date_parse_time() {
        match Date::parse(&String::from("19361020T120000"), &String::new()).unwrap() {
            Date::Time(time) => {
                assert_eq!(time.year(), 1936);
                assert_eq!(time.hour(), 12);
                assert_eq!(time.day(), 20);
            }
            _ => assert!(true),
        }
    }
}
