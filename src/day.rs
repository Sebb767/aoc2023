use std::cmp::min;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

pub type BoxedDay = Box<dyn Day>;

pub type DayResult = i128;

#[derive(Copy, Clone)]
pub struct YearDay {
    pub year: u16,
    pub day: u16,
}

pub struct ExpectedResults {
    pub part1_test: DayResult,
    pub part1_real: Option<DayResult>,
    pub part2_test: Option<DayResult>,
    pub part2_real: Option<DayResult>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RunType {
    Test,
    Actual,
}

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1 = 1u16,
    Part2 = 2u16,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RunResultType {
    Failed = 0,
    Unverified = 1,
    Success = 2,
}

#[macro_export]
macro_rules! ExpectedResult {
    ($x: expr) => { ExpectedResults::create_expected_results($x, None, None, None) };
    ($x: expr, $y: expr) => { ExpectedResults::create_expected_results($x, Some($y), None, None) };
    ($x: expr, $y: expr, $z: expr) => { ExpectedResults::create_expected_results($x, Some($y), Some($z), None) };
    ($x: expr, $y: expr, $z: expr, $w: expr) => { ExpectedResults::create_expected_results($x, Some($y), Some($z), Some($w)) };
}

impl ExpectedResults {
    pub fn create_expected_results(
        part1_test: DayResult,
        part1_real: Option<DayResult>,
        part2_test: Option<DayResult>,
        part2_real: Option<DayResult>) -> ExpectedResults {
        ExpectedResults {
            part1_test,
            part1_real,
            part2_test,
            part2_real,
        }
    }

    pub fn get_expected_result_for_type(&self, run_type: &RunType, part: &Part) -> Option<DayResult> {
        match (run_type, part) {
            (RunType::Test, Part::Part1) => Some(self.part1_test),
            (RunType::Actual, Part::Part1) => self.part1_real,
            (RunType::Test, Part::Part2) => self.part2_test,
            (RunType::Actual, Part::Part2) => self.part2_real,
        }
    }
}

impl YearDay {
    pub fn y2024(day: u16) -> Self {
        YearDay { year: 2024, day }
    }
}

impl Display for RunType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RunType::Test => String::from("test"),
            RunType::Actual => String::from("real"),
        };
        write!(f, "{}", str)
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}

pub trait Day {
    fn part1(&self, input: String) -> Option<DayResult>;
    fn part2(&self, input: String) -> Option<DayResult>;
    fn get_expected_results(&self) -> ExpectedResults;
    fn get_year_and_date(&self) -> YearDay;

    fn part1_result_description(&self) -> String {
        String::from("Result for part 1")
    }

    fn part2_result_description(&self) -> String {
        String::from("Result for part 2")
    }

    fn get_input(&self, run_type: &RunType, part: &Part) -> Option<String> {
        let YearDay { year, day } = self.get_year_and_date();
        let prefix = format!("./inputs/{}/", year);
        let mut suffixes = vec!(
            format!("day{day}-{part}.{run_type}.txt"),
            format!("day{day}.{run_type}.txt"),
        );

        if *run_type == RunType::Actual {
            suffixes.push(format!("day{day}-{part}.txt"));
            suffixes.push(format!("day{day}.txt"));

            if *part == Part::Part2 {
                suffixes.push(format!("day{day}-1.txt"));
            }
        }

        for suffix in suffixes {
            if let Ok(input) = fs::read_to_string(format!("{prefix}/{suffix}")) {
                return Some(input);
            }
        }

        None
    }

    fn run_part(&self, run_type: &RunType, part: &Part) -> RunResultType {
        let result_description = match part {
            Part::Part1 => self.part1_result_description(),
            Part::Part2 => self.part2_result_description(),
        };

        let input = self.get_input(run_type, part);
        if input.is_none() {
            println!("Part {part} failed - could not find input!");
            return RunResultType::Failed;
        }
        let input = input.unwrap();

        if let Some(result) = match part {
            Part::Part1 => self.part1(input),
            Part::Part2 => self.part2(input),
        } {
            if let Some(expected) = self.get_expected_results().get_expected_result_for_type(run_type, part) {
                if result == expected {
                    println!("{result_description}: {result} (verified)");
                    RunResultType::Success
                } else {
                    println!("{result_description}: {result}");
                    println!(" => FAILED! {result} != {expected}");
                    RunResultType::Failed
                }
            } else {
                println!("{}: {} (no reference value given)", result_description, result);
                RunResultType::Unverified
            }
        } else {
            println!("Failed: Day function did not return a valid result!");
            RunResultType::Failed
        }
    }

    fn run_type(&self, run_type: &RunType) -> RunResultType {
        let YearDay { year, day } = self.get_year_and_date();
        let spacer = "=========";
        println!("{spacer} {year}, Day {day} ({run_type}) {spacer}");

        let start = Instant::now();
        let result = min(
            self.run_part(run_type, &Part::Part1),
            self.run_part(run_type, &Part::Part2),
        );
        let elapsed = start.elapsed().as_secs_f64();

        println!();
        println!("# day {day} {run_type} completed in {elapsed:.3}s");
        result
    }

    fn run(&self) -> RunResultType {
        let test = self.run_type(&RunType::Test);
        println!();
        let actual = self.run_type(&RunType::Actual);

        min(test, actual)
    }
}