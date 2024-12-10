use std::fmt::{Display, Formatter};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

    pub fn diff(&self, other: &Position) -> PositionDelta {
        PositionDelta {
            x: (self.x as CoordinateDelta) - (other.x as CoordinateDelta),
            y: (self.y as CoordinateDelta) - (other.y as CoordinateDelta),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
    }
}


#[derive(Debug)]
pub struct PositionDelta {
    x: CoordinateDelta,
    y: CoordinateDelta,
}

impl PositionDelta {
    pub fn reverse(&self) -> PositionDelta {
        PositionDelta {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Display for PositionDelta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Δ(x={}, y={})", self.x, self.y)
    }
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
#[rustfmt::skip] // :>
pub enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Direction::North, Direction::East, Direction::West, Direction::South];

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
            Direction::North => &[Direction::West, Direction::East, Direction::South],
            Direction::West => &[Direction::North, Direction::East, Direction::South],
            Direction::East => &[Direction::West, Direction::North, Direction::South],
            Direction::South => &[Direction::West, Direction::East, Direction::North],
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y)?.get(x)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y)?.get_mut(x)
    }

    pub fn get_position(&self, position: &Position) -> Option<&T> {
        self.get(position.x, position.y)
    }

    pub fn get_position_mut(&mut self, position: &Position) -> Option<&mut T> {
        self.get_mut(position.x, position.y)
    }


    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.xsize, self.ysize)
    }

    pub fn checked_position_apply(&self, position: &Position, delta: &PositionDelta) -> Option<Position> {
        let next = position.apply(delta)?;
        if self.validate_position(&next) {
            Some(next)
        } else {
            None
        }
    }

    pub fn validate_position(&self, position: &Position) -> bool {
        position.x < self.xsize && position.y < self.ysize
    }

    pub fn from_string<F>(input: &String, mapper: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let lines: Vec<&str> = input.lines().collect();
        let mut data: Vec<Vec<T>> = Vec::with_capacity(lines.capacity());

        for line in lines {
            data.push(
                line
                    .chars()
                    .map(&mapper)
                    .collect()
            );
        }

        Matrix::create(data)
    }
}

impl<T> Display for Matrix<T> where T : Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix {}×{}\n", self.xsize, self.ysize)?;
        for row in self.data.iter() {
            for point in row.iter() {
                write!(f, "{}", point)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<T> Matrix<T>
where
    T: Eq,
{
    pub fn find_first(&self, needle: &T) -> Option<Position> {
        for (y, row) in self.data.iter().enumerate() {
            if let Some(x) = row.iter().position(|elem| elem == needle) {
                return Some(Position { x, y });
            }
        }

        None
    }
}

impl Matrix<char> {
    pub fn char_matrix_from_string(input: &String) -> Self {
        Self::from_string(input, |c| c)
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = (Position, &'a T);
    type IntoIter = MatrixIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let (xsize, _) = self.get_dimensions();
        MatrixIterator {
            matrix: self,
            xsize,
            ypos: 0,
            xpos: 0,
        }
    }
}

pub struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    xpos: usize,
    ypos: usize,
    xsize: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.matrix.get(self.xpos, self.ypos)?;
        let position = Position { x: self.xpos, y: self.ypos };

        self.xpos += 1;
        if self.xpos >= self.xsize {
            self.xpos = 0;
            self.ypos += 1;
        }

        Some((position, item))
    }
}