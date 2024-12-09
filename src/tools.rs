use std::fs;
use std::str::FromStr;

pub fn get_input(day: &str, year: u16) -> Result<String, String> {
    let fname = format!("./inputs/{}/day{}.txt", year, day);
    let result = fs::read_to_string(fname.clone());
    if let Ok(result) = result {
        Ok(result)
    } else {
        Err(format!("Could not read file {}!", fname))
    }
}

pub fn get_input_or_panic(day: &str, year: u16) -> String {
    let result = get_input(day, year);
    if result.is_err() {
        panic!("{}", result.unwrap_err());
    };
    result.unwrap()
}

pub fn string_with_spaces_and_numbers_to_vec_of_numbers<T: FromStr>(
    input: &str,
) -> Result<Vec<T>, T::Err> {
    let parsed: Result<Vec<T>, T::Err> = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect();
    parsed
}

pub fn input_with_lines_with_spaces_and_numbers_to_vec_of_vec_of_numbers<T: FromStr>(
    input: &str
) -> Result<Vec<Vec<T>>, T::Err> {
    input
        .lines()
        .map(|l| string_with_spaces_and_numbers_to_vec_of_numbers(l))
        .collect()
}


#[macro_export]
macro_rules! return_none_unless {
    ($val:expr $(,)?) => {
        match $val {
            true => (),
            false => return None,
        }
    };
}


pub type Coordinate = usize;
pub type CoordinateDelta = isize;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Position {
    pub fn apply(&self, delta: &PositionDelta) -> Option<Position> {
        let x = self.x.checked_add_signed(delta.x)?;
        let y = self.y.checked_add_signed(delta.y)?;
        Some(Position { x, y })
    }
}

#[derive(Debug)]
pub struct PositionDelta {
    x: CoordinateDelta,
    y: CoordinateDelta,
}

#[derive(Debug, Hash, Copy, Clone)]
#[rustfmt::skip] // :>
pub enum Direction {
    North,
    West,      East,
    South,
}

impl Direction {
    pub const ALL : [Direction; 4] = [Direction::North, Direction::East, Direction::West, Direction::South];

    pub fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
        }
    }

    pub fn to_position_delta(&self) -> PositionDelta {
        match self {
            Direction::North => PositionDelta { x: 0, y: -1 },
            Direction::West => PositionDelta { x: -1, y: 0 },
            Direction::East => PositionDelta { x: 1, y: 0 },
            Direction::South => PositionDelta { x: 0, y: 1 },
        }
    }

    pub fn get_others(&self) -> &[Direction; 3] {
        match self {
            Direction::North => &[ Direction::West, Direction::East, Direction::South ],
            Direction::West => &[ Direction::North, Direction::East, Direction::South ],
            Direction::East => &[ Direction::West, Direction::North, Direction::South ],
            Direction::South => &[ Direction::West, Direction::East, Direction::North ],
        }
    }
}

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
    xsize: usize,
    ysize: usize,
}

impl<T> Matrix<T> {
    pub fn create(data: Vec<Vec<T>>) -> Self {
        let ysize = data.len();
        assert_ne!(ysize, 0);
        let xsize = data.get(0).unwrap().len();
        data.iter().for_each(|v| assert_eq!(xsize, v.len()));
        Matrix { data, xsize, ysize }
    }

    pub fn get(&self, x : usize, y : usize) -> Option<&T> {
        return self.data.get(y)?.get(x);
    }

    pub fn get_data(&self) -> &Vec<Vec<T>> {
        &self.data
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.xsize, self.ysize)
    }
}