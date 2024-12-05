use std::vec;
use crate::aoc2023::day1::day1;
use crate::aoc2023::day2::day2;
use crate::aoc2023::day3::day3;
use crate::aoc2023::day4::day4;
use crate::aoc2023::day5::day5;
use crate::aoc2023::day6::day6;
use crate::aoc2023::day7::day7;
use crate::aoc2023::day8::day8;
use crate::aoc2023::day9::day9;
use crate::aoc2023::day10::Day10;
use crate::aoc2023::fallback::Fallback;
use crate::day::BoxedDay;

mod day1;
mod day10;
mod day2;
mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
mod fallback;

pub fn get_days() -> Vec<fn()>
{
    let days = vec!(day1, day2, day3, day4, day5, day6, day7, day8, day9);
    return days;
}

pub fn get_days_adv() -> Vec<BoxedDay> {
    let mut basic: Vec<BoxedDay> = get_days().iter().enumerate().map(|dt| {
        let (idx, closure) = dt;
        Fallback::create_boxed((idx + 1) as u16, *closure)
    }).collect();
    basic.push(Box::new(Day10));
    basic
}