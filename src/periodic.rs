use std::fmt;
use std::str::FromStr;

use event::Event;
use errors::EventError;

#[derive(Debug)]
pub struct Periodic {
    pub event: Event,
    pub freq: Freq,
    // TODO: until, count, interval, ...
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
        }
    }

    pub fn set_param(&mut self, param: &str, value: &str) -> Result<(), EventError> {
        match param {
            "FREQ" => self.freq = value.parse()?,
            _ => (),
        }
        Ok(())
    }
}

impl fmt::Display for Periodic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.freq, self.event)?;
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
