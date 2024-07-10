use std::collections::HashMap;
use crate::tools::get_input_or_panic;

type Coordinate = usize;
type CoordinateDelta = isize;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl Position {
    fn apply(&self, delta: &PositionDelta) -> Option<Position> {
        let x = self.x.checked_add_signed(delta.x)?;
        let y = self.y.checked_add_signed(delta.y)?;
        Some(Position { x, y })
    }
}

#[derive(Debug)]
struct PositionDelta {
    x: CoordinateDelta,
    y: CoordinateDelta,
}

#[derive(Debug, Hash, Copy, Clone)]
#[rustfmt::skip] // :>
enum Direction {
         North,
    West,      East,
         South,
}

impl Direction {
    const ALL : [Direction; 4] = [Direction::North, Direction::East, Direction::West, Direction::South];

    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
        }
    }

    fn to_position_delta(&self) -> PositionDelta {
        match self {
            Direction::North => PositionDelta { x: 0, y: -1 },
            Direction::West => PositionDelta { x: -1, y: 0 },
            Direction::East => PositionDelta { x: 1, y: 0 },
            Direction::South => PositionDelta { x: 0, y: 1 },
        }
    }

    fn get_others(&self) -> &[Direction; 3] {
        match self {
            Direction::North => &[ Direction::West, Direction::East, Direction::South ],
            Direction::West => &[ Direction::North, Direction::East, Direction::South ],
            Direction::East => &[ Direction::West, Direction::North, Direction::South ],
            Direction::South => &[ Direction::West, Direction::East, Direction::North ],
        }
    }
}

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
        let delta = direction.to_position_delta();
        // this will fail if we try to move below 0 on either axis
        let new_position = position.apply(&delta)?;
        // if this fails, we're out of bounds
        self.lookup(&new_position)?;

        // last, we need to check whether the pipes connect
        if self.is_open_to(position, direction) && self.is_open_to(&new_position, &direction.reverse()) {
            return Some(new_position);
        }

        None
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

pub fn day10() {
    day10_1();
    day10_2();
}

fn day10_1() {
    let input = get_input_or_panic("10-1");
    let grid = Grid::new(input);
    let distance = find_distance_to_farthest_point(&grid);


    println!("Distance to farthest point: {}", distance);
}

fn day10_2() {
    let _input = get_input_or_panic("10-1");
}
