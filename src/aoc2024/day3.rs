use regex::Regex;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

pub(crate) struct Day3;

impl Day for Day3 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0i128;

        for (_, [factor1, factor2]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
            let factor1: i128 = factor1.parse().ok()?;
            let factor2: i128 = factor2.parse().ok()?;
            sum += factor1 * factor2;
        }

        Some(sum)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do|don't)\(\)").unwrap();
        let mut sum = 0i128;
        let mut active = true;

        for capture in re.captures_iter(input.as_str()) {
            let mut iter = capture.iter();
            iter.next();
            let factor1 = iter.next().unwrap().map_or(None, |m| { Some(m.as_str()) });
            let factor2 = iter.next().unwrap().map_or(None, |m| { Some(m.as_str()) });
            let op = iter.next().unwrap().map_or(None, |m| { Some(m.as_str()) });

            match (factor1, factor2, op, active) {
                (Some(factor1), Some(factor2), None, true) => {
                    let factor1: i128 = factor1.parse().ok()?;
                    let factor2: i128 = factor2.parse().ok()?;
                    sum += factor1 * factor2;
                }
                (Some(_), Some(_), None, false) => {}
                (None, None, Some("don't"), _) => active = false,
                (None, None, Some("do"), _) => active = true,
                _ => {
                    dbg!(format!("{:#?} {:#?} {:#?} {active}", factor1, factor2, op));
                }
            }
        }

        Some(sum)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(161, 187833789, 48)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(3)
    }

    fn part1_result_description(&self) -> String {
        String::from("Result of valid multiplication instructions")
    }

    fn part2_result_description(&self) -> String {
        String::from("Result of valid multiplication instructions without disabled ones")
    }
}