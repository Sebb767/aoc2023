use std::fs;
use std::str::FromStr;

pub fn get_input(day: &str) -> Result<String, String> {
    let fname = format!("./inputs/day{}.txt", day);
    let result = fs::read_to_string(fname.clone());
    if let Ok(result) = result {
        Ok(result)
    } else {
        Err(format!("Could not read file {}!", fname))
    }
}

pub fn get_input_or_panic(day: &str) -> String {
    let result = get_input(day);
    if result.is_err() {
        panic!("{}", result.unwrap_err());
    };
    result.unwrap()
}

pub fn string_with_spaces_and_numbers_to_vec_of_numbers<T: FromStr>(
    input: &str,
) -> Result<Vec<T>, T::Err> {
    let parsed: Result<Vec<T>, T::Err> = input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse())
        .collect();
    parsed
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
