use std::collections::{HashMap, HashSet};
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResultMultipleTests;
use crate::tools::{Matrix, Position, PositionDelta};

pub(crate) struct Day8;

fn input_to_antennas_and_matrix(input: &String) -> (Matrix<char>, HashMap<char, Vec<Position>>) {
    let matrix = Matrix::char_matrix_from_string(&input);
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    for (pos, sign) in matrix.into_iter() {
        if *sign != '.' {
            if let Some(list) = antennas.get_mut(sign) {
                list.push(pos);
            } else {
                antennas.insert(*sign, vec!(pos));
            }
        }
    }

    (matrix, antennas)
}

fn iter_while_valid_position<T>(matrix: &Matrix<T>, nodes: &mut HashSet<Position>,start: &Position, direction: &PositionDelta) {
    if let Some(next) = matrix.checked_position_apply(start, direction) {
        iter_while_valid_position(matrix, nodes, &next, direction);
        nodes.insert(next);
    }
}

impl Day for Day8 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let (matrix, antennas) = input_to_antennas_and_matrix(&input);
        let mut antinodes: HashSet<Position> = HashSet::new();

        for (_, antennas_of_type) in antennas.iter() {
            for a in antennas_of_type.iter() {
                for b in antennas_of_type.iter() {
                    if a != b {
                        let diff = a.diff(b);

                        if let Some(first_antinode) = matrix.checked_position_apply(a, &diff) {
                            antinodes.insert(first_antinode);
                        }
                        if let Some(second_antinode ) = matrix.checked_position_apply(b, &diff.reverse()) {
                            antinodes.insert(second_antinode);
                        }
                    }
                }
            }
        }

        Some(antinodes.len() as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let (matrix, antennas) = input_to_antennas_and_matrix(&input);
        let mut antinodes: HashSet<Position> = HashSet::new();

        for (_, antennas_of_type) in antennas.iter() {
            for a in antennas_of_type.iter() {
                for b in antennas_of_type.iter() {
                    if a != b {
                        let diff = a.diff(b);
                        // Note that we reversed the Antenna starting points, as this will include
                        // the antennas itself as antinodes. We could do so separately, but then
                        // we'd need to check whether it's the only antenna of the type and this is
                        // a simple solution.
                        iter_while_valid_position(&matrix, &mut antinodes, b, &diff);
                        iter_while_valid_position(&matrix, &mut antinodes, a, &diff.reverse());
                    }
                }
            }
        }

        /*for node in antinodes.iter() {
            if let Some(mut pos) = matrix.get_position_mut(node) {
                *pos = '#';
            }
        }
        println!("{}", matrix);*/

        Some(antinodes.len() as DayResult)

    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResultMultipleTests!(vec!(14), 359, vec!(34, 9))
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(8)
    }

    fn part1_result_description(&self) -> String {
        String::from("Number of antinode locations")
    }

    fn part2_result_description(&self) -> String {
        String::from("Number of extended antinode locations")
    }
}