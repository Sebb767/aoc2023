use std::collections::HashMap;
use std::hash::Hash;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

pub(crate) struct Day1;

impl Day for Day1 {
    fn part1(&self, input: String) -> Option<DayResult> {
        Some(day1_1(input))
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        Some(day1_2(input))
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(0, 2375403, 0, 23082277)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay {
            year: 2024,
            day: 1,
        }
    }

    fn part1_result_description(&self) -> String {
        String::from("Sum of differences of numbers")
    }

    fn part2_result_description(&self) -> String {
        String::from("Difference score")
    }
}

fn read_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<&str> = input.lines().collect();
    let count = lines.len();

    let mut left: Vec<u64> = Vec::with_capacity(count);
    let mut right: Vec<u64> = Vec::with_capacity(count);

    for line in lines.into_iter() {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(numbers.len(), 2);
        let mut iter = numbers.into_iter();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();
    assert_eq!(left.len(), right.len());

    return (left, right);
}

fn day1_1(input: String) -> DayResult {
    let (left, right) = read_input(input);

    let mut ileft = left.into_iter();
    let mut iright = right.into_iter();
    let mut result = 0u64;
    while let Some(nleft) = ileft.next() {
        let nright = iright.next().unwrap();
        result += u64::abs_diff(nright, nleft);
    }

    result as DayResult
}

fn list_to_occurrence_map<T>(inp: &Vec<T>) -> HashMap<T, u64>
where
    T: Hash + Eq + Copy,
{
    let mut map: HashMap<T, u64> = HashMap::new();

    for num in inp.into_iter() {
        if let Some(kref) = map.get_mut(&num) {
            *kref += 1;
        } else {
            map.insert(*num, 1);
        }
    }

    map
}

fn day1_2(input: String) -> DayResult {
    let (left, right) = read_input(input);
    let lmap = list_to_occurrence_map(&left);
    let rmap = list_to_occurrence_map(&right);

    let mut result = 0u64;
    for key in lmap.keys() {
        let amount_right = *lmap.get(key).unwrap();
        if let Some(amount_left) = rmap.get(key) {
            result += *key * amount_right * *amount_left;
        }
    }

    result as DayResult
}