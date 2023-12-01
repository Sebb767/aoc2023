use std::fs;

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