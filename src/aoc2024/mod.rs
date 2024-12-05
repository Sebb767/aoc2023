use std::vec;
use crate::aoc2024::day1::Day1;
use crate::aoc2024::day2::Day2;
use crate::day::{BoxedDay, Day};

mod day1;
mod day2;

pub fn get_days() -> Vec<fn()>
{
    let days: Vec<fn()> = vec!(|| { Day1.run(); }, || { Day2.run(); });
    return days;
}

pub fn get_days_adv() -> Vec<BoxedDay> {
    let days: Vec<BoxedDay> = vec!(
        Box::new(Day1), 
        Box::new(Day2)
    );
    days
}