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
    let mut args = env::args().skip(1);
    let (first, last) = period(&args.next().unwrap());

    let calendars: Vec<_> = args.map(|arg| ics_calendar(&arg)).collect();
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

fn period(arg: &str) -> (Date, Date) {
    let days = match arg {
        "day" => 1,
        "week" => 7,
        "month" => 30,
        _ => panic!("Invalid time frame, try: day, week or month"),
    };
    let first = Date::now();
    let last = first + Duration::days(days);
    (first, last)
}

fn ics_calendar(file_path: &str) -> Calendar {
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    Calendar::parse(buf).unwrap()
}
