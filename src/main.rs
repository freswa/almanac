extern crate ical;
extern crate chrono;
extern crate chrono_tz;

mod date;
mod event;
mod periodic;
mod calendar;
mod errors;

use std::env;
use std::io::BufReader;
use std::fs::File;
use chrono::Duration;
use date::Date;
use calendar::Calendar;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let buf = BufReader::new(file);
    let calendar = Calendar::parse(buf).unwrap();
    println!("{}", calendar);
    println!("");

    let now = Date::now();
    let events = calendar.get(&now, &(now + Duration::weeks(10)));
    for e in events {
        println!("{}", e);
    }
}
