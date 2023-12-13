mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod reduce;
mod tools;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;
use std::env;
use std::time::Instant;

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

fn main() {
    let days = [day1, day2, day3, day4, day5, day6, day7, day8, day9];
    let args: Vec<String> = env::args().collect();

    if args.len() > 0 && args.contains(&String::from("--all")) {
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
    } else {
        let current_day = days.last().unwrap();
        call_day(current_day, days.len());
    }
}
