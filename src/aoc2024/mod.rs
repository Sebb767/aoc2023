use std::vec;
use crate::aoc2024::day1::day1;
use crate::aoc2024::day2::day2;

mod day1;
mod day2;

pub fn get_days() -> Vec<fn()>
{
    let days : Vec<fn()> = vec!(day1, day2);
    return days;
}