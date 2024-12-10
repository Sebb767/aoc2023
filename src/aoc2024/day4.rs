use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;
use crate::tools::Matrix;

pub(crate) struct Day4;

impl Day for Day4 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let lines: Vec<&str> = input.lines().collect();
        let chars: Vec<Vec<char>> = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let matrix = Matrix::create(chars);
        let (xsize, ysize) = matrix.get_dimensions();
        let mut result = 0i128;

        for x in 0..xsize {
            for y in 0..ysize {
                if matrix.get(x, y) == Some(&'X') {
                    for dx in -1..2 {
                        for dy in -1..2 {
                            let x = x as i32;
                            let y = y as i32;
                            if matrix.get((x + dx) as usize, (y + dy) as usize) == Some(&'M') &&
                                matrix.get((x + dx * 2) as usize, (y + dy * 2) as usize) == Some(&'A') &&
                                matrix.get((x + dx * 3) as usize, (y + dy * 3) as usize) == Some(&'S') {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }

        Some(result)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let lines: Vec<&str> = input.lines().collect();
        let chars: Vec<Vec<char>> = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let matrix = Matrix::create(chars);
        let (xsize, ysize) = matrix.get_dimensions();
        let mut result = 0i128;

        for x in 1..(xsize - 1) {
            for y in 1..(ysize - 1) {
                if matrix.get(x, y) == Some(&'A') {
                    if is_valid_xmas(
                        vec!(
                            matrix.get(x-1, y-1),
                            matrix.get(x+1, y-1),
                            matrix.get(x+1, y+1),
                            matrix.get(x-1, y+1),
                        )) {
                        result += 1;
                    }
                }
            }
        }

        Some(result)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(18, 2536, 9, 1875)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(4)
    }

    fn part1_result_description(&self) -> String {
        String::from("Number of 'XMAS' in the puzzle")
    }

    fn part2_result_description(&self) -> String {
        String::from("Number of 'X-MAS' in the puzzle")
    }
}

fn is_valid_xmas(input: Vec<Option<&char>>) -> bool {
    assert_eq!(input.len(), 4);
    let mut m = 0;
    let mut s = 0;
    for char in input.iter() {
        if let Some(char) = char {
            match char {
                &'M' => m += 1,
                &'S' => s += 1,
                &_ => { return false; }
            }
        } else {
            return false;
        }
    }

    m == 2 && s == 2 && input.windows(2).any(|tpl| {
        if let [i1, i2] = tpl {
            return *i1.unwrap() == *i2.unwrap();
        }
        return false
    })
}