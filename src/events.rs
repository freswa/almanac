use std::io::BufRead;
use std::fmt;
use ical::IcalParser;

use event::{Event, Date};
use periodic::Periodic;
use errors::EventError;

pub struct Events {
    single: Vec<Event>,
    periodic: Vec<Periodic>,
}

impl Events {
    pub fn parse<B: BufRead>(buf: B) -> Result<Self, EventError> {
        let reader = IcalParser::new(buf);
        let mut single = Vec::new();
        let mut periodic = Vec::new();

        for line in reader {
            for ev in line?.events {
                let mut event = Event::new();
                let mut maybe_periodic = None;

                for property in ev.properties {
                    let value = property.value.unwrap_or("".to_string());
                    let mut time_zone = "".to_string();

                    let params = property.params.unwrap_or(vec![]);
                    for (param, value) in &params {
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
                        "RRULE" => maybe_periodic = Some(rrule(&value, &params)?),
                        _ => (),
                    };
                }
                match maybe_periodic {
                    Some(mut p) => {
                        p.event = event;
                        periodic.push(p);
                    }
                    None => single.push(event),
                }
            }
        }

        single.sort_by(|a, b| a.start.cmp(&b.start));
        Ok(Events { single, periodic })
    }
}

impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for event in &self.single {
            writeln!(f, "{}", event)?;
        }
        writeln!(f, "")?;
        for periodic in &self.periodic {
            writeln!(f, "{}", periodic)?;
        }
        Ok(())
    }
}

fn rrule(value: &String, params: &Vec<(String, Vec<String>)>) -> Result<Periodic, EventError> {
    let mut periodic = Periodic::new();

    let p: Vec<&str> = value.splitn(2, "=").collect();
    periodic.set_param(p[0], p[1])?;

    for (param, values) in params {
        let mut value = "";
        if values.len() > 0 {
            value = &values[0];
        }
        periodic.set_param(param, value)?;
    }

    Ok(periodic)
}
