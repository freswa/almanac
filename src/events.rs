use std::io::BufRead;
use std::fmt;
use ical::IcalParser;

use event::{Event, Date};
use errors::EventError;

pub struct Events {
    single: Vec<Event>,
}

impl Events {
    pub fn parse<B: BufRead>(buf: B) -> Result<Self, EventError> {
        let reader = IcalParser::new(buf);
        let mut single = Vec::new();

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
                        "DTSTART" => event.start = Date::parse(&value, &time_zone)?,
                        "DTEND" => event.end = Date::parse(&value, &time_zone)?,
                        _ => (),
                    };
                }
                single.push(event);
            }
        }

        single.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(Events { single })
    }
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for event in &self.single {
            writeln!(f, "{}", event)?;
        }
        Ok(())
    }
}
