use crate::tools::get_input_or_panic;

#[allow(dead_code)]
pub fn day1() {
    day1_1(); // Sum: 53334
}

pub fn day1_1() {
    let input = get_input_or_panic("1-1");

    let mut sum : i32 = 0;
    for line in input.lines() {
        let result = parse_line(line);
        if result.is_some() {
            let result = result.unwrap();
            sum += result;
            println!("Line {} -> {} (sum: {})", line, result, sum);
        }
        else {
            println!("Line {} -> ()", line);
        }
    }

    println!("Final sum: {}", sum);
}

fn parse_line(line : &str) -> Option<i32> {
    let digits = "0123456789";
    let mut first : Option<i32> = Option::None;
    let mut last : Option<i32> = Option::None;

    for char in line.chars() {
        let digit_result = digits.find(char);
        if digit_result.is_some() {
            let digit : Option<i32> = Some(digit_result.unwrap() as i32);
            if first.is_none() {
                first = digit.clone();
                last = digit;
            }
            else {
                last = digit;
            }
        }
    }

    if first.is_some() {
        if last.is_some() {
            Option::from((first.unwrap() * 10) + last.unwrap())
        }
        else {
            first
        }
    }
    else {
        Option::None
    }
}

