mod reduce;
mod tools;
mod aoc2023;
mod aoc2024;
mod day;

use std::collections::HashMap;
use std::env;
use std::process::exit;
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
    let days_raw = vec!(
        aoc2023::get_days_adv(),
        aoc2024::get_days_adv()
    );
    let mut days : Vec<&BoxedDay> = days_raw.iter().flatten().collect();

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter();
    let argv0 = args_iter.next().unwrap();
    let mut filter_years: Option<Vec<u16>> = None;
    let mut filter_days: Option<Vec<u16>> = None;
    let mut latest_only = false;

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--year" => {
                if let Some(year_selection) = args_iter.next() {
                    let possible_filter_years: Result<Vec<u16>, _> = year_selection
                        .split(',')
                        .map(|s| s.parse())
                        .collect();

                    if let Ok(possible_filter_years) = possible_filter_years {
                        filter_years = Some(possible_filter_years);
                    } else {
                        println!("Error: Could not parse '{year_selection}' as year list!");
                        print_help(argv0);
                        exit(1);
                    }
                } else {
                    println!("Error: years parameter without specified years!");
                    print_help(argv0);
                    exit(1);
                }
            }
            "--day" => {
                if let Some(day_selection) = args_iter.next() {
                    let possible_filter_days: Result<Vec<u16>, _> = day_selection
                        .split(',')
                        .map(|s| s.parse())
                        .collect();

                    if let Ok(possible_filter_days) = possible_filter_days {
                        filter_days = Some(possible_filter_days);
                    } else {
                        println!("Error: Could not parse '{day_selection}' as day list!");
                        print_help(argv0);
                        exit(1);
                    }
                } else {
                    println!("Error: days parameter without specified days!");
                    print_help(argv0);
                    exit(1);
                }
            }
            "--latest" => { latest_only = true; }
            unknown => {
                println!("Error: Unknown parameter '{unknown}'!");
                print_help(argv0);
                exit(1);
            }
        }
    }

    days.retain(|day| {
        let YearDay { year, day } = day.get_year_and_date();
        if filter_years.as_ref().is_some() && !filter_years.as_ref().unwrap().contains(&year) {
            return false;
        }
        if filter_days.as_ref().is_some() && !filter_days.as_ref().unwrap().contains(&day) {
            return false;
        }
        true
    });

    days.sort_by(|a, b| a.get_year_and_date().cmp(&b.get_year_and_date()));

    if days.is_empty() {
        println!("No days found matching current filter!");
        exit(2);
    }

    if latest_only {
        let current_day = days.last().unwrap();
        call_day(current_day);
    } else {
        run_list(&days);
    }
}

fn print_help(argv0: &String)
{
    println!("Usage: {argv0} [--year n[,n,...]] [--day n[,n,...]] --latest");
    println!("  --year n[,n,...]");
    println!("    Comma-separated list of years to run.");
    println!("  --day n[,n,...]");
    println!("    Comma-separated list of days to run.");
    println!("  --latest");
    println!("    Only run the latest day in the latest year. Can be combined with year filter to run the last day of a specific year.");
}

fn run_list(input: &Vec<&BoxedDay>) {
    let mut header_year : u16 = 0;
    
    let now = Instant::now();
    let mut results: Vec<RunResultType> = Vec::new();
    let mut first = true;

    for day in input.into_iter() {
        let YearDay { year, .. } = day.get_year_and_date();
        if year != header_year {
            print_year_header(year);
            header_year = year;
        }
        else {
            if first {
                first = false;
            }
            else {
                println!();
            }
        }
        results.append(&mut call_day(day));
    }

    print!(
        "# Whole run finished after {:.3}s ; ",
        now.elapsed().as_secs_f64()
    );
    print_results(&results);
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

    for key in [RunResultType::Success, RunResultType::Unverified, RunResultType::Failed] {
        print!("{:?}={} ", key, frequencies.get(&key).unwrap_or(&0));
    }
    println!();
}