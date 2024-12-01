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
use crate::aoc2023::day10::day10;

pub(crate) mod day1;
pub(crate) mod day10;
pub(crate) mod day2;
pub(crate) mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn get_days() -> Vec<fn()>
{
    let days = vec!(day1, day2, day3, day4, day5, day6, day7, day8, day9, day10);
    return days;
}