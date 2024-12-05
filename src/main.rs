mod reduce;
mod tools;
mod aoc2023;
mod aoc2024;
mod day;

use std::collections::HashMap;
use std::env;
use std::time::Instant;
use crate::day::{BoxedDay, YearDay};

fn call_day(day: &BoxedDay)
{
    let YearDay { year: _, day: nday } = day.get_year_and_date();
    let start = Instant::now();

    day.run();

    let elapsed = start.elapsed().as_secs_f64();
    println!("# day {nday} completed in {elapsed:.3}s");
}

fn print_year_header(year: u16) {
    println!("#############################################");
    println!("###   {}", year);
    println!("#############################################");
}

fn main() {
    let years = HashMap::from([
        (2023u16, aoc2023::get_days_adv()),
        (2024u16, aoc2024::get_days_adv())
    ]);

    let args: Vec<String> = env::args().collect();

    if !args.is_empty() && args.contains(&String::from("--all")) {
        let mut keys: Vec<&u16> = years.keys().collect();
        keys.sort();
        let now = Instant::now();

        for key in keys.into_iter() {
            let days = years.get(key).unwrap();
            print_year_header(*key);
            run_days(days);
        }

        println!(
            "# All days finished after {:.3}s",
            now.elapsed().as_secs_f64()
        )
    } else {
        let max_year = years.keys().max().unwrap();
        let days = years.get(max_year).unwrap();
        let current_day = days.last().unwrap();
        call_day(current_day);
    }
}

fn run_days(days: &Vec<BoxedDay>) {
    let mut first = true;

    for day in days.iter() {
        if first {
            first = false
        } else {
            println!();
        }
        call_day(day);
    }
}
