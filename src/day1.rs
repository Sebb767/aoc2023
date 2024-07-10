use crate::tools::get_input_or_panic;

#[allow(dead_code)]
pub fn day1() {
    assert_eq!(parse_line_advanced("2zf2"), Some(22i32));
    day1_1(); // Sum: 53334
    day1_2(); // Sum: 52834
}

pub fn day1_1() {
    let input = get_input_or_panic("1-1");
    let sum = summer(input, parse_line);

    println!("Final sum (part 1): {}", sum);
    assert_eq!(sum, 53334);
}

pub fn day1_2() {
    let input = get_input_or_panic("1-1");
    let sum = summer(input, parse_line_advanced);

    println!("Final sum (part 2): {}", sum);
    assert_eq!(sum, 52834);
}

fn summer<F>(input: String, line_parser: F) -> i32
where
    F: Fn(&str) -> Option<i32>,
{
    let mut sum: i32 = 0;
    for line in input.lines() {
        let result = line_parser(line);
        if result.is_some() {
            let result = result.unwrap();
            sum += result;
            //println!("Line {} -> {} (sum: {})", line, result, sum);
        } else {
            println!("Line {} -> ()", line);
        }
    }

    sum
}

// Technically, we'd also need to return how much to advance the string by
// But no digit as word will be valid with the first character cut, so we can always lazily advance
// by one.
fn string_to_next_digit(input: &str) -> Option<i32> {
    assert!(!input.is_empty());

    let digit_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = "0123456789";

    if let Some(idx) = digits.find(input.chars().next().unwrap()) {
        return Some(idx as i32);
    };

    for (idx, digit_word) in digit_words.iter().enumerate() {
        if input.starts_with(digit_word) {
            return Option::Some(idx as i32);
        }
    }
    None
}

fn parse_line_advanced(line: &str) -> Option<i32> {
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for offset in 0..line.len() {
        let digit = string_to_next_digit(&line[offset..]);
        if digit.is_some() {
            if first.is_none() {
                first = digit;
                last = digit;
            } else {
                last = digit;
            }
        }
    }

    combine_digits(first, last)
}

fn parse_line(line: &str) -> Option<i32> {
    let digits = "0123456789";
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;

    for char in line.chars() {
        let digit_result = digits.find(char);
        if digit_result.is_some() {
            let digit: Option<i32> = Some(digit_result.unwrap() as i32);
            if first.is_none() {
                first = digit;
            } else {
                last = digit;
            }
        }
    }

    combine_digits(first, last)
}

fn combine_digits(first: Option<i32>, last: Option<i32>) -> Option<i32> {
    let fdigit = first?;
    let result = match last {
        Some(ldigit) => (fdigit * 10) + ldigit,
        None => fdigit * 11,
    };
    Some(result)
}
