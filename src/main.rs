mod reduce;
mod tools;
mod aoc2023;

use std::collections::HashMap;
use std::env;
use std::time::Instant;
use crate::aoc2023::get_days;

fn call_day<F>(function: &F, day: usize)
where
    F: Fn(),
{
    let spacer = "=========";
    println!("{spacer} Day {day} {spacer}");

    let start = Instant::now();
    function();
    let elapsed = start.elapsed().as_secs_f64();

    println!();
    println!("# day {day} completed in {elapsed:.3}s");
}

fn print_year_header(year : u16) {
    println!("#############################################");
    println!("###   {}", year);
    println!("#############################################");
}

fn main() {
    let years = HashMap::from([
        (2023u16, get_days())
    ]);

    let args: Vec<String> = env::args().collect();

    if !args.is_empty() && args.contains(&String::from("--all")) {
        let keys = years.keys();
        for key in keys.into_iter() {
            let days = years.get(key).unwrap();
            print_year_header(*key);
            run_days(days);
        }

    } else {
        let max_keys = years.keys().last().unwrap();
        let days = years.get(max_keys).unwrap();
        let current_day = days.last().unwrap();
        call_day(current_day, days.len());
    }
}

fn run_days(days : &Vec<fn()>) {
    let mut first = true;
    let now = Instant::now();

    for (day, function) in days.iter().enumerate() {
        if first {
            first = false
        } else {
            println!();
            println!();
        }
        call_day(function, day + 1);
    }

    println!(
        "# All days finished after {:.3}s",
        now.elapsed().as_secs_f64()
    )
}
