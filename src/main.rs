extern crate almanac;

use std::env;
use std::io::BufReader;
use std::fs::File;
use almanac::Duration;
use almanac::Date;
use almanac::Calendar;

fn main() {
    let args: Vec<_> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let buf = BufReader::new(file);
    let calendar = Calendar::parse(buf).unwrap();

    let now = Date::now();
    let events = calendar.get(&now, &(now + Duration::days(1)));
    for e in events {
        println!("{}", e);
    }
}
