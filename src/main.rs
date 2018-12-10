extern crate almanac;
extern crate itertools;

use std::env;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

use almanac::Calendar;
use almanac::Date;
use almanac::Duration;

fn main() {
    let first = Date::now();
    let last = first + Duration::days(7);

    let calendars: Vec<_> = env::args().skip(1).map(|arg| ics_calendar(&arg)).collect();
    let events = calendars
        .iter()
        .map(|c| c.iter())
        .kmerge()
        .skip_while(|e| e.end_date() < first)
        .take_while(|e| e.start <= last);

    for event in events {
        println!("{}", event);
    }
}

fn ics_calendar(file_path: &str) -> Calendar {
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    Calendar::parse(buf).unwrap()
}
