extern crate almanac;
extern crate itertools;
extern crate colored;

use std::env;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;
use colored::*;

use almanac::Calendar;
use almanac::Date;
use almanac::Duration;
use almanac::Event;
use almanac::Config;

fn main() {
    let conf = Config::parse().unwrap_or(Config::new());
    let mut args = env::args().skip(1);
    let period_arg = match args.next() {
        Some(arg) => arg,
        None => {
            if conf.period.is_empty() {
                println!("Usage: almanac day|week|month [ical ...]");
                return;
            } else {
                conf.period
            }
        }
    };
    let (first, last) = period(&period_arg);

    let mut calendars: Vec<_> = args.map(|arg| ics_calendar(&arg)).collect();
    if calendars.is_empty() {
        for cal in &conf.cals {
            calendars.push(ics_calendar(cal))
        }
    }

    let events = calendars
        .iter()
        .map(|c| c.iter())
        .kmerge()
        .skip_while(|e| e.end_date() < first)
        .take_while(|e| e.start <= last);
    print_events(events)
}

fn period(arg: &str) -> (Date, Date) {
    let days = match arg {
        "all" => 0,
        "day" => 1,
        "week" => 7,
        "month" => 30,
        _ => panic!("Invalid time frame, try: all, day, week or month"),
    };
    if days == 0 {
        (Date::new(), Date::max())
    } else {
        let first = Date::now();
        (first, first + Duration::days(days))
    }
}

fn ics_calendar(file_path: &str) -> Calendar {
    let file = File::open(file_path).unwrap();
    let buf = BufReader::new(file);
    Calendar::parse(buf).unwrap()
}

fn print_events(events: impl Iterator<Item = Event>) {
    let mut day = Date::new();
    let mut unfinish: Vec<Event> = vec![];

    for event in events {
        if !day.same_day(&event.start) {
            if !unfinish.is_empty() {
                while !day.same_day(&event.start) {
                    day = day + Duration::days(1);
                    print_day(day);
                    for (i, event) in unfinish.clone().iter().enumerate() {
                        if event.end_date() <= day + Duration::days(1) {
                            unfinish.remove(i);
                            print_event(event, true, false);
                        } else {
                            print_event(event, true, true);
                        }
                    }
                }
            } else {
                day = event.start.clone();
                print_day(day);
            }
        }

        if event.end_date() > event.start + Duration::days(1) {
            print_event(&event, false, true);
            unfinish.push(event);
        } else {
            print_event(&event, false, false);
        }
    }

    while !unfinish.is_empty() {
        day = day + Duration::days(1);
        print_day(day);
        for (i, event) in unfinish.clone().iter().enumerate() {
            if event.end_date() <= day + Duration::days(1) {
                unfinish.remove(i);
                print_event(event, true, false);
            } else {
                print_event(event, true, true);
            }
        }
    }
    println!("");
}

fn print_day(date: Date) {
    println!("\n{}", date.format("%a %b %e %Y").green().bold())
}

fn print_event(event: &Event, ustart: bool, uend: bool) {
    let start = if ustart {
        "-----".to_string()
    } else {
        match event.start {
            Date::Time(_) => event.start.format("%R"),
            Date::AllDay(_) => "-----".to_string(),
        }
    };
    let end = if uend {
        "-----".to_string()
    } else {
        match event.end_date() {
            Date::Time(_) => event.end_date().format("%R"),
            Date::AllDay(_) => "-----".to_string(),
        }
    };

    println!(
        "    {}-{} {} {}",
        start.yellow(),
        end.yellow(),
        event.summary,
        event.location.purple()
    );

    if !event.description.is_empty() {
        let description = str::replace(&event.description, "\\n", &format!("\n{}", " ".repeat(16)));
        println!("{}{}", " ".repeat(16), description.cyan());
    }
}
