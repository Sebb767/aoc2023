use std::collections::HashMap;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

use crate::tools::string_with_spaces_and_numbers_to_vec_of_numbers;

pub(crate) struct Day11;

type Stone = u64;
type StoneRow = Vec<Stone>;
const LOOKUP: [u64; 19] = [
    10u64,
    100u64,
    1000u64,
    10000u64,
    100000u64,
    1000000u64,
    10000000u64,
    100000000u64,
    1000000000u64,
    10000000000u64,
    100000000000u64,
    1000000000000u64,
    10000000000000u64,
    100000000000000u64,
    1000000000000000u64,
    10000000000000000u64,
    100000000000000000u64,
    1000000000000000000u64,
    10000000000000000000u64,
];

fn num_digits(stone: &Stone) -> u64 {
    let mut digits = 1;
    for pow in LOOKUP {
        if stone >= &pow {
            digits += 1;
        }
        else {
            break;
        }
    }

    digits
}

fn split(stone: &Stone) -> Option<(Stone, Stone)> {
    let digits = num_digits(stone);
    // 2 -> AB div 10 10^1
    // 4 -> AABB div 100 10^2
    // 6 -> AAABBB div 1000 10^3
    if digits % 2 == 0 {
        let halflen = digits / 2;
        let div = LOOKUP[(halflen - 1) as usize];
        Some((
            stone / div,
            stone % div,
        ))
    }
    else {
        None
    }
}

fn score(stone: &Stone, remaining_blinks: u32, mut cache: &mut HashMap<(Stone, u32), DayResult>) -> DayResult {
    if remaining_blinks == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(*stone, remaining_blinks)) {
        return *result;
    }

    let fscore = if stone == &0 {
        score(&1, remaining_blinks - 1, &mut cache)
    }
    else if let Some((a, b)) = split(stone) {
        score(&a, remaining_blinks - 1, &mut cache) +
            score(&b, remaining_blinks - 1, &mut cache)
    }
    else {
        score(&(stone * 2024), remaining_blinks - 1, &mut cache)
    };

    cache.insert((*stone, remaining_blinks), fscore);
    fscore
}

fn blink_multiple(row: StoneRow, blinks: u32) -> DayResult {
    let mut cache = HashMap::new();

    row.iter().map(|s| score(s, blinks, &mut cache)).sum()
}

impl Day for Day11 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let row : StoneRow = string_with_spaces_and_numbers_to_vec_of_numbers(&input).ok()?;
        Some(blink_multiple(row, 25))
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let row : StoneRow = string_with_spaces_and_numbers_to_vec_of_numbers(&input).ok()?;
        Some(blink_multiple(row, 75))
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(55312, 212655, 65601038650482, 253582809724830)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(11)
    }

    fn part1_result_description(&self) -> String {
        String::from("Number of stones after 25 blinks")
    }

    fn part2_result_description(&self) -> String {
        String::from("Number of stones after 75 blinks")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(num_digits(&0), 1);
        assert_eq!(num_digits(&1), 1);
        assert_eq!(num_digits(&9), 1);

        assert_eq!(num_digits(&10), 2);
        assert_eq!(num_digits(&50), 2);
        assert_eq!(num_digits(&99), 2);

        assert_eq!(num_digits(&100), 3);
        assert_eq!(num_digits(&500), 3);
        assert_eq!(num_digits(&999), 3);

        assert_eq!(num_digits(&1000), 4);
        assert_eq!(num_digits(&5000), 4);
        assert_eq!(num_digits(&9999), 4);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(&1), None);
        assert_eq!(split(&111), None);
        assert_eq!(split(&999), None);

        assert_eq!(split(&10), Some((1, 0)));
        assert_eq!(split(&12), Some((1, 2)));
        assert_eq!(split(&2024), Some((20, 24)));
    }

}