use std::collections::HashMap;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResultMultipleTests;
use crate::tools::{Direction, Position};
/*
       -y
       |
       |
 -x ---*--- (+x)
       |
       |
       +y

 So our lookup is nodes[y][x]!
*/
struct Grid {
    nodes: Vec<Vec<char>>,
    start_position: Position,
}

impl Grid {
    fn new(input: String) -> Grid {
        let mut outer = Vec::new();
        let mut start_position : Option<Position> = None;

        for line in input.lines() {
            let inner : Vec<char> = line.chars().collect();
            if let Some(s_pos_x) = inner.iter().position(|&c| c == 'S') {
                if start_position.is_some() {
                    panic!("Found two starting positions!");
                }
                start_position = Some(Position {
                    x: s_pos_x,
                    y: outer.len(),
                });
            }
            outer.push(inner);
        }

        if start_position.is_none() {
            panic!("Found no starting position!");
        }

        Grid { nodes: outer, start_position: start_position.unwrap() }
    }

    fn lookup(&self, position: &Position) -> Option<&char> {
        self.nodes
            .get(position.y)?
            .get(position.x)
    }

    fn move_checked(&self, position: &Position, direction: &Direction) -> Option<Position> {
        let new_position = self.move_unchecked(position, direction)?;

        // last, we need to check whether the pipes connect
        if self.is_open_to(position, direction) && self.is_open_to(&new_position, &direction.reverse()) {
            return Some(new_position);
        }

        None
    }

    fn move_unchecked(&self, position: &Position, direction: &Direction) -> Option<Position> {
        let delta = direction.to_position_delta();
        // this will fail if we try to move below 0 on either axis
        let new_position = position.apply(&delta)?;
        // if this fails, we're out of bounds
        //dbg!(format!("delta={:?} dir={:?}", delta, direction));
        self.lookup(&new_position)?;
        return Some(new_position);
    }

    fn is_open_to(&self, position: &Position, direction: &Direction) -> bool {
        let char = self.lookup(position);
        if char.is_none() {
            return false;
        }
        matches!(
            (*char.unwrap(), direction),
            ('S', _)
                | ('|', Direction::North | Direction::South)
                | ('-', Direction::East | Direction::West)
                | ('L', Direction::North | Direction::East)
                | ('J', Direction::North | Direction::West)
                | ('7', Direction::South | Direction::West)
                | ('F', Direction::South | Direction::East)
        )
    }
}

fn find_distance_to_farthest_point(grid : &Grid) -> usize {
    type VisitQueueEntry = (
        Position, // The position
        Direction, // where we came from (from the view of THIS node)
        usize // how many steps it took
    );
    type VisitedEntry = (
        Direction, // where we came from (from the view of THIS node)
        usize // how many steps it took
    );
    let mut to_visit : Vec<VisitQueueEntry> = Vec::new();
    let mut visited : HashMap<Position, VisitedEntry> = HashMap::new();

    Direction::ALL.iter().for_each(|direction| {
        if let Some(newpows) = grid.move_checked(&grid.start_position, direction) {
            to_visit.push((newpows, direction.reverse(), 1));
        }
    });

    while let Some(current) = to_visit.pop() {
        let (position, entering_direction, offset) = current;
        if let Some(other_way) = visited.get(&position) {
            // we found the loop!
            // We don't need to follow it, knowing the lengths of both paths is sufficient
            let total_loop_length = offset + other_way.1;
            assert_eq!(total_loop_length % 2, 0);
            let farthest_point = total_loop_length / 2;
            return farthest_point;
        }

        entering_direction.get_others().iter().for_each(|direction| {
            if let Some(newpows) = grid.move_checked(&position, direction) {
                to_visit.push((newpows, direction.reverse(), offset + 1));
            }
        });
        visited.insert(position, (entering_direction, offset));
    }

    unreachable!();
}

fn find_loop(grid : &Grid) -> HashMap<Position,Direction> {
    type VisitQueueEntry = (
        Position, // The position
        Direction, // where we came from (from the view of THIS node)
        usize // how many steps it took
    );
    type VisitedEntry = (
        Direction, // where we came from (from the view of THIS node)
        usize // how many steps it took Direction
    );
    let mut to_visit : Vec<VisitQueueEntry> = Vec::new();
    let mut visited : HashMap<Position, VisitedEntry> = HashMap::new();

    Direction::ALL.iter().for_each(|direction| {
        if let Some(newpows) = grid.move_checked(&grid.start_position, direction) {
            // setting the node to visited here is technically not perfect, since the start point
            // could lead to three directions, but none of the inputs do actually have that scenario
            //visited.insert(grid.start_position.clone(), (Direction::South, 0));
            to_visit.push((newpows, direction.reverse(), 1));
        }
    });

    while let Some(current) = to_visit.pop() {
        let (position, entering_direction, offset) = current;
        if let Some((other_diction, _)) = visited.get(&position) {
            // we found the loop!
            let mut result : HashMap<Position, Direction> = HashMap::new();

            // first, insert the current node
            result.insert(position.clone(), entering_direction);

            let mut cursor = position.clone();
            let mut direction = entering_direction;
            // then find the way back to the start
            loop {
                let nextpos = grid.move_unchecked(&cursor, &direction.reverse());
                if nextpos.is_none() {
                    println!();
                    let direction_rev = direction.reverse();
                    grid.move_checked(&cursor, &direction_rev);
                }
                let nextpos = nextpos.unwrap();
                if let Some((new_direction, _)) = visited.get(&nextpos) {
                    result.insert(nextpos.clone(), new_direction.clone());
                    cursor = nextpos;
                    direction = *new_direction;
                }
                else {
                    // at the start
                    break;
                }
            }

            // reset, go the other way
            cursor = position;
            direction = *other_diction;
            loop {
                let nextpos = grid.move_unchecked(&cursor, &direction.reverse()).unwrap();
                if let Some((new_direction, _)) = visited.get(&nextpos) {
                    result.insert(nextpos.clone(), new_direction.clone().reverse());
                    cursor = nextpos;
                    direction = *new_direction;
                }
                else {
                    // at the start
                    result.insert(nextpos, direction.reverse());
                    break;
                }
            }


            return result;

        }

        entering_direction.get_others().iter().for_each(|direction| {
            if let Some(newpows) = grid.move_checked(&position, direction) {
                to_visit.push((newpows, direction.reverse(), offset + 1));
            }
        });
        visited.insert(position, (entering_direction, offset));
    }

    unreachable!();
}

pub(crate) struct Day10;
impl Day for Day10 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let grid = Grid::new(input);
        let distance = find_distance_to_farthest_point(&grid) - 1;

        Some(distance as DayResult)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let grid = Grid::new(input);
        return None;
        let gloop = find_loop(&grid);
        let mut blocks_in_loop = 0;

        for x in 0..grid.nodes.len() {
            let mut in_loop = false;
            for y in 0..grid.nodes.get(0).unwrap().len() {
                let pos = Position { x, y };
                if gloop.contains_key(&pos) {
                    in_loop = !in_loop;
                }
                else if in_loop {
                    blocks_in_loop += 1;
                }
            }
        }

        Some(blocks_in_loop)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResultMultipleTests!(vec!(8), 6733, vec!(4, 8, 10))
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay { year: 2023, day: 10 }
    }

    fn part1_result_description(&self) -> String {
        String::from("Distance to farthest point")
    }

    /*fn part2_result_description(&self) -> String {
        todo!()
    }*/
}
