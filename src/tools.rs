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
