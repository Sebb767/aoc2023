use std::collections::HashSet;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResultMultipleTests;
use crate::tools::{Direction, Matrix, Position};

pub(crate) struct Day10;

fn follow_trail(matrix: &Matrix<u32>, peaks: &mut HashSet<Position>, position: &Position, height: u32) -> DayResult {
    let p1 = height + 1;
    let mut result = 0;

    for direction in Direction::ALL {
        if let Some(nextpos) = matrix.checked_position_apply(&position, &direction.to_position_delta()) {
            let neighbour_height = matrix.get_position(&nextpos).unwrap();
            if neighbour_height == &p1 {
                if p1 == 9 {
                    peaks.insert(nextpos);
                    result += 1;
                } else {
                    result += follow_trail(matrix, peaks, &nextpos, p1);
                }
            }
        }
    }

    result
}

impl Day for Day10 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let matrix: Matrix<u32> = Matrix::from_string(&input, |char| char.to_digit(10).unwrap_or(99));
        let mut result = 0;

        for (pos, height) in matrix.into_iter() {
            if height == &0 {
                let mut peaks = HashSet::new();
                follow_trail(&matrix, &mut peaks, &pos, 0);
                result += peaks.len();
            }
        }

        Some(result as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let matrix: Matrix<u32> = Matrix::from_string(&input, |char| char.to_digit(10).unwrap_or(99));
        let mut result = 0;

        for (pos, height) in matrix.into_iter() {
            if height == &0 {
                let mut peaks = HashSet::new();
                result += follow_trail(&matrix, &mut peaks, &pos, 0);
            }
        }

        Some(result as DayResult)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResultMultipleTests!(vec!(36, 1), 737, vec!(81, 3, 13, 227), 1619)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(10)
    }

    fn part1_result_description(&self) -> String {
        String::from("Sum of scores of trailheads")
    }

    fn part2_result_description(&self) -> String {
        String::from("Sum of scores of trailheads by path count")
    }
}