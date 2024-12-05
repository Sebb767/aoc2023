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