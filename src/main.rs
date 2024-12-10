mod reduce;
mod tools;
mod aoc2023;
mod aoc2024;
mod day;

use std::collections::HashMap;
use std::env;
use std::time::Instant;
use crate::day::{BoxedDay, RunResultType, YearDay};

fn call_day(day: &BoxedDay) -> Vec<RunResultType>
{
    let YearDay { year: _, day: nday } = day.get_year_and_date();
    let start = Instant::now();

    let result = day.run();

    let elapsed = start.elapsed().as_secs_f64();
    println!("# day {nday} completed in {elapsed:.3}s");
    result
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
        run_list(&years);
    } else {
        let max_year = years.keys().max().unwrap();
        let days = years.get(max_year).unwrap();
        let current_day = days.last().unwrap();
        call_day(current_day);
    }
}

fn run_list(input: &HashMap<u16, Vec<BoxedDay>>) {
    let mut keys: Vec<&u16> = input.keys().collect();
    keys.sort();
    let now = Instant::now();
    let mut results: Vec<RunResultType> = Vec::new();

    for key in keys.into_iter() {
        let days = input.get(key).unwrap();
        print_year_header(*key);
        let mut day_results = run_days(days);
        results.append(&mut day_results);
    }

    print!(
        "# Whole run finished after {:.3}s ; ",
        now.elapsed().as_secs_f64()
    );
    print_results(&results);
}

fn run_days(days: &Vec<BoxedDay>) -> Vec<RunResultType> {
    let mut first = true;
    let mut results = Vec::new();

    for day in days.iter() {
        if first {
            first = false
        } else {
            println!();
        }
        results.append(&mut call_day(day));
    }

    results
}

fn print_results(results: &Vec<RunResultType>) {
    let frequencies = results
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, val| {
            map.entry(val)
                .and_modify(|frq| *frq += 1)
                .or_insert(1);
            map
        });

    for key in [ RunResultType::Success, RunResultType::Unverified, RunResultType::Failed ] {
        print!("{:?}={} ", key, frequencies.get(&key).unwrap());
    }
    println!();
}