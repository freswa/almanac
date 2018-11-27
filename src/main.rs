extern crate almanac;

use std::env;
use std::io::BufReader;
use std::fs::File;

use almanac::Calendar;
use almanac::Date;
use almanac::Duration;
use almanac::Event;

fn main() {
    let first = Date::now();
    let last = first + Duration::days(7);

    let mut events: Vec<_> = env::args()
        .skip(1)
        .map(|arg| ics_calendar(&arg, &first, &last))
        .flatten()
        .collect();
    events.sort();
    for event in events {
        println!("{}", event);
    }
}

fn ics_calendar(file_path: &str, first: &Date, last: &Date) -> Vec<Event> {
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    let calendar = Calendar::parse(buf).unwrap();
    calendar.get(first, last)
}
