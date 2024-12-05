use std::time::Instant;
use crate::day::{BoxedDay, Day, DayResult, ExpectedResults, Part, RunResultType, RunType, YearDay};

pub struct Fallback {
    year_day: YearDay,
    closure: fn()
}

impl Fallback {
    fn create(day : u16, closure : fn()) -> Self {
        Self {
            year_day: YearDay {
                year: 2023,
                day,
            },
            closure,
        }
    }

    pub fn create_boxed(day : u16, closure : fn()) -> BoxedDay {
        Box::new(Self::create(day, closure))
    }
}

impl Day for Fallback {
    fn part1(&self, _: String) -> Option<DayResult> {
        unimplemented!()
    }

    fn part2(&self, _: String) -> Option<DayResult> {
        unimplemented!()
    }

    fn get_expected_results(&self) -> ExpectedResults {
        unimplemented!()
    }

    fn get_year_and_date(&self) -> YearDay {
        self.year_day
    }

    fn run_part(&self, _: &RunType, _: &Part) -> RunResultType {
        unimplemented!();
    }

    fn run_type(&self, run_type: &RunType) -> RunResultType {
        if *run_type == RunType::Test {
            unimplemented!()
        }

        let YearDay { year, day } = self.get_year_and_date();
        let spacer = "=========";
        println!("{spacer} {year}, Day {day} ({run_type}) {spacer}");

        let start = Instant::now();
        (self.closure)();
        let elapsed = start.elapsed().as_secs_f64();

        println!();
        println!("# day {day} {run_type} completed in {elapsed:.3}s");
        println!();
        RunResultType::Unverified
    }

    fn run(&self) -> RunResultType {
        let result = self.run_type(&RunType::Actual);
        println!();
        result
    }
}