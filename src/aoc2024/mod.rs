use std::vec;
use crate::aoc2024::day1::day1;

mod day1;
pub fn get_days() -> Vec<fn()>
{
    let days : Vec<fn()> = vec!({ day1 });
    return days;
}