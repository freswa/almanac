extern crate ical;
extern crate chrono;
extern crate chrono_tz;

mod event;
mod events;
mod errors;

use std::env;
use std::io::BufReader;
use std::fs::File;
use events::Events;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let buf = BufReader::new(file);
    let events = Events::parse(buf).unwrap();
    println!("{}", events);
}
