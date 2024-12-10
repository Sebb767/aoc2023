use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

pub(crate) struct Day7;

impl Day for Day7 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let lines: Vec<Vec<u64>> = input.lines()
            .map(|line| {
                line.split(&[' ', ':'])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect()
            }).collect();

        Some(lines.iter().map(is_valid_line).sum::<u64>() as DayResult)
    }


    fn part2(&self, input: String) -> Option<DayResult> {
        let lines: Vec<Vec<u64>> = input.lines()
            .map(|line| {
                line.split(&[' ', ':'])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect()
            }).collect();

        Some(lines.iter().map(is_valid_line2).sum::<u64>() as DayResult)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(3749, 5512534574980, 11387, 328790210468594)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(7)
    }

    fn part1_result_description(&self) -> String {
        String::from("Sum of valid equation results")
    }

    fn part2_result_description(&self) -> String {
        String::from("Sum of valid equation results with concat op")
    }
}

fn is_valid_line(line: &Vec<u64>) -> u64 {
    let mut iter = line.iter();
    let value = iter.next().unwrap();
    let first = iter.next().unwrap();

    if is_valid_line_iter(*value, *first, &iter.map(|e| *e).collect()) {
        return *value;
    }
    return 0;
}

fn is_valid_line_iter(expected: u64, sum: u64, rest: &Vec<u64>) -> bool {
    if rest.len() == 0 {
        return expected == sum;
    }
    let mut iter = rest.iter();
    let current = iter.next().unwrap();
    let rest: Vec<u64> = iter.map(|x| *x).collect();

    is_valid_line_iter(expected, sum + current, &rest) || is_valid_line_iter(expected, sum * current, &rest)
}


fn is_valid_line2(line: &Vec<u64>) -> u64 {
    let mut iter = line.iter();
    let value = iter.next().unwrap();
    let first = iter.next().unwrap();

    if is_valid_line_iter2(*value, *first, &iter.map(|e| *e).collect()) {
        return *value;
    }
    return 0;
}

fn is_valid_line_iter2(expected: u64, sum: u64, rest: &Vec<u64>) -> bool {
    if rest.len() == 0 {
        return expected == sum;
    }
    let mut iter = rest.iter();
    let current = iter.next().unwrap();
    let rest: Vec<u64> = iter.map(|x| *x).collect();

    is_valid_line_iter2(expected, sum + current, &rest)
        || is_valid_line_iter2(expected, sum * current, &rest)
        || is_valid_line_iter2(expected, concat_numbers(sum, *current), &rest)
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}