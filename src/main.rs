extern crate ical;
extern crate chrono;
extern crate chrono_tz;

use std::env;
mod ics;
mod event;

fn main() {
    let args: Vec<_> = env::args().collect();
    let events = ics::parse(&args[1]).unwrap();
    for event in events {
        println!("{}", event);
    }
}
