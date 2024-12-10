use std::collections::HashSet;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;
use crate::tools::{Direction, Matrix, Position};

pub(crate) struct Day6;

fn walk_matrix(matrix: &Matrix<char>) -> Option<HashSet<(Position, Direction)>> {
    let mut position = matrix.find_first(&'^').unwrap();
    let mut direction = Direction::North;
    let mut visited_dupe : HashSet<(Position, Direction)> = HashSet::new();
    visited_dupe.insert((position.clone(), direction));

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
                if ! visited_dupe.insert((next.clone(), direction)) {
                    // dupe detected -> we have a loop!
                    return None;
                }
                position = next;
            },
            c => {
                dbg!(format!("Found invalid char {c} at {:?}!", next));
                unimplemented!();
            }
        }
    }

    Some(visited_dupe)
}

fn unique_visited_locations(inp : HashSet<(Position, Direction)>) -> HashSet<Position> {
    let mut rv = HashSet::new();
    inp.iter().for_each(|(pos, _)| { rv.insert(pos.clone()); });
    rv
}

impl Day for Day6 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let matrix = Matrix::char_matrix_from_string(&input);
        let visited = unique_visited_locations(walk_matrix(&matrix).unwrap());

        Some(visited.len() as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let mut matrix = Matrix::char_matrix_from_string(&input);
        let visited = unique_visited_locations(walk_matrix(&matrix).unwrap());
        let mut possible_loops = 0;
        let v_iter = visited.iter();
        //let _start_position = v_iter.next().unwrap(); // we can't place a crate at the start

        for pos in v_iter {
            let pref = matrix.get_position_mut(&pos).unwrap();
            let old = *pref;
            if old == '^' {
                continue;
            }
            *pref = '#';
            if None == walk_matrix(&matrix) {
                // if it loops, we get none here
                possible_loops += 1;
            }
            *matrix.get_position_mut(&pos).unwrap() = old;
        }

        Some(possible_loops)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(41, 4559, 6, 1604)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(6)
    }

    fn part1_result_description(&self) -> String {
        String::from("Number of distinct squares the guard visited")
    }

    fn part2_result_description(&self) -> String {
        String::from("Number of squares where you could put an object to loop the guard")
    }
}