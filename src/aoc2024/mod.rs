use std::vec;
use crate::aoc2024::day1::Day1;
use crate::aoc2024::day2::Day2;
use crate::aoc2024::day3::Day3;
use crate::aoc2024::day4::Day4;
use crate::aoc2024::day5::Day5;
use crate::aoc2024::day6::Day6;
use crate::aoc2024::day7::Day7;
use crate::day::{BoxedDay};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn get_days_adv() -> Vec<BoxedDay> {
    let days: Vec<BoxedDay> = vec!(
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
        Box::new(Day7),
    );
    days
}