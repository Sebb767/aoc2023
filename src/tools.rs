use std::fs;
use std::str::FromStr;

pub fn get_input(day : &str) -> Result<String, String> {
    let fname = format!("./inputs/day{}.txt", day);
    let result = fs::read_to_string(fname.clone());
    if result.is_ok() {
        Ok(result.unwrap())
    }
    else {
        Err(format!("Could not read file {}!", fname))
    }
}

pub fn get_input_or_panic(day : &str) -> String {
    let result = get_input(day);
    if result.is_err() {
        panic!("{}", result.unwrap_err());
    };
    return result.unwrap();
}

pub fn string_with_spaces_and_numbers_to_vec_of_numbers<T : FromStr>(input : &str) -> Result<Vec<T>, T::Err> {
    let parsed : Result<Vec<T>, T::Err> = input.split(" ").map(|s| s.parse()).collect();
    return parsed;
}