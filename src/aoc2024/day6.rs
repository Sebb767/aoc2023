use std::collections::HashSet;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;
use crate::tools::{Direction, Matrix, Position};

pub(crate) struct Day6;

fn walk_matrix(matrix: &Matrix<Waypoint>) -> Option<HashSet<(Position, Direction)>> {
    let mut position = matrix.into_iter().filter (|(_, wp)| wp.sign == '^').next().unwrap().0;
    let mut direction = Direction::North;
    let mut visited_dupe : HashSet<(Position, Direction)> = HashSet::new();
    visited_dupe.insert((position.clone(), direction));

    while let Some(next) = matrix.checked_position_apply(&position, &direction.to_position_delta()) {
        let space = matrix.get_position(&next).unwrap();

        match space.sign {
            '#' => {
                direction = direction_to_right(&direction)
            },
            '.'|'^' => {
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

fn reset_matrix_waypoints(matrix: &mut Matrix<Waypoint>) {
    let (xmax, ymax) = matrix.get_dimensions();
    for y in 0..ymax {
        for x in 0..xmax {
            matrix.get_mut(x, y).unwrap().reset();
        }
    }
}

fn direction_to_right(direction: &Direction) -> Direction {
    match direction {
        Direction::North => { Direction::East }
        Direction::East => { Direction::South }
        Direction::South => { Direction::West }
        Direction::West => { Direction::North }
    }
}

fn check_for_loop(mut matrix: &mut Matrix<Waypoint>, starting_position: Position) -> bool {
    let mut position = starting_position;
    let mut direction = Direction::North;
    let test = matrix.get_position_mut(&position).unwrap().visit(&direction);
    assert_eq!(test, false);

    while let Some(next) = matrix.checked_position_apply(&position, &direction.to_position_delta()) {
        let space = matrix.get_position_mut(&next).unwrap();

        match space.sign {
            '#' => {
                direction = direction_to_right(&direction)
            },
            '.'|'^' => {
                if space.visit(&direction) {
                    // found a loop!
                    reset_matrix_waypoints(&mut matrix);
                    return true;
                }
                position = next;
            },
            c => {
                dbg!(format!("Found invalid char {c} at {:?}!", next));
                unimplemented!();
            }
        }
    }

    reset_matrix_waypoints(&mut matrix);
    false
}

struct Waypoint {
    sign: char,
    visited_dir: [Option<Direction>; 4]
}

impl Waypoint {
    fn create(sign: char) -> Self {
        Self {
            sign,
            visited_dir: [ None; 4 ],
        }
    }

    /// Returns whether the location was already visited from this direction
    fn visit(&mut self, direction: &Direction) -> bool {
        for i in 0..self.visited_dir.len() {
            match self.visited_dir[i] {
                None => {
                    self.visited_dir[i] = Some(*direction);
                    return false;
                }
                Some(dir) => {
                    if &dir == direction {
                        return true;
                    }
                }
            }
        }

        unreachable!();
    }

    fn reset(&mut self) {
        self.visited_dir = [ None; 4 ];
    }
}

fn unique_visited_locations(inp : HashSet<(Position, Direction)>) -> HashSet<Position> {
    let mut rv = HashSet::new();
    inp.iter().for_each(|(pos, _)| { rv.insert(pos.clone()); });
    rv
}

impl Day for Day6 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let matrix = Matrix::from_string(&input, Waypoint::create);
        let visited = unique_visited_locations(walk_matrix(&matrix).unwrap());

        Some(visited.len() as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let mut matrix = Matrix::from_string(&input, Waypoint::create);
        let visited = unique_visited_locations(walk_matrix(&matrix).unwrap());
        let mut possible_loops = 0;
        let start_position = matrix.into_iter().filter (|(_, wp)| wp.sign == '^').next().unwrap().0;

        for pos in visited.iter() {
            let pref = matrix.get_position_mut(&pos).unwrap();
            let old = pref.sign;
            if old == '^' {
                continue;
            }
            pref.sign = '#';
            if check_for_loop(&mut matrix, start_position.clone()) {
                // if it loops, we get none here
                possible_loops += 1;
            }
            matrix.get_position_mut(&pos).unwrap().sign = old;
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

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_waypoint() {
        let mut wp = Waypoint::create('x');

        assert_eq!(wp.visited_dir, [None; 4]);
        assert_eq!(wp.visit(&Direction::North), false);
        assert_eq!(wp.visit(&Direction::North), true);
        assert_eq!(wp.visit(&Direction::South), false);
        assert_eq!(wp.visit(&Direction::South), true);
        assert_eq!(wp.visited_dir, [ Some(Direction::North), Some(Direction::South), None, None ]);
        wp.reset();
        assert_eq!(wp.visited_dir, [None; 4]);
    }
}