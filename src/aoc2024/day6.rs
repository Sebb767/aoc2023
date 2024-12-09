use std::collections::HashSet;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;
use crate::tools::{Direction, Matrix, Position};

pub(crate) struct Day6;

impl Day for Day6 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let matrix = Matrix::char_matrix_from_string(&input);
        let mut position = matrix.find_first(&'^').unwrap();
        let mut direction = Direction::North;
        let mut visited : HashSet<Position> = HashSet::new();
        visited.insert(position.clone());

        while let Some(next) = matrix.checked_position_apply(&position, &direction.to_position_delta()) {
            let space = matrix.get_position(&next).unwrap();

            match space {
                &'#' => {
                    match direction {
                        Direction::North => { direction = Direction::East }
                        Direction::East => { direction = Direction::South }
                        Direction::South => { direction = Direction::West }
                        Direction::West => { direction = Direction::North }
                    }
                },
                &'.'|&'^' => {
                    visited.insert(next.clone());
                    position = next;
                },
                c => {
                    dbg!(format!("Found invalid char {c} at {:?}!", next));
                    unimplemented!();
                }
            }
        }


        Some(visited.len() as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        None
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(41, 4559, 6)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(6)
    }

    fn part1_result_description(&self) -> String {
        String::from("Number of distinct squares the guard visited:")
    }

    /*fn part2_result_description(&self) -> String {
        todo!()
    }*/
}