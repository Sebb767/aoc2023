use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use crate::tools::get_input_or_panic;

#[derive(Debug, Copy, Clone)]
struct Number {
    value : u32,
    x : usize,
    y : usize,
    length : usize
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
    x : usize,
    y : usize
}

struct Field {
    numbers : Vec<Number>,
    symbols : Vec<Symbol>
}

/// If the current item is a digit, advance the iterator until the number is fully parsed and return
/// the number. Otherwise, return none.
fn parse_number(first_digit : &char, x : usize, y : usize, iterator : &mut Peekable<Enumerate<Chars>>) -> Option<Number> {
    let digit = first_digit.to_digit(10);
    if digit.is_none() {
        return None
    }
    let mut value = digit.unwrap();
    let mut length : usize = 1;

    // advance the iterator until we found all digits
    // We use peak as to not consume the next symbol
    while let Some((_ ,char)) = iterator.peek() {
        let number = (*char).to_digit(10);
        if number.is_some() {
            value = (value * 10) + number.unwrap();
            length += 1;
            // advance the iterator, consuming the digit we just parsed
            iterator.next();
        }
        else {
            break;
        }
    }

    return Some(Number {
        value,
        x,
        y,
        length
    });
}


fn parse_field(input : String) -> Field {
    let mut numbers : Vec<Number> = Vec::new();
    let mut symbols : Vec<Symbol> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut chars = line.chars().into_iter().enumerate().peekable();

        while let Some((x, char)) = chars.next() {
            if let Some(number) = parse_number(&char, x, y, &mut chars) {
                numbers.push(number);
            } else if char != '.' {
                symbols.push(Symbol {
                    x,
                    y
                });
            }
        }
    }

    return Field {
        numbers,
        symbols
    };
}

/// A number is a part number if any symbol is adjacent to it, even diagonally
fn check_if_number_is_part_number(number : &Number, symbols : &Vec<Symbol>) -> bool {
    let ymin = if number.y == 0 { 0 } else { number.y - 1 };
    let ymax = number.y + 1;
    let xmin = if number.x == 0 { 0 } else { number.x - 1 };
    // Since the length is always 1>, this includes a +1 already
    let xmax = number.x + number.length;

    return symbols.into_iter().any(
        |symbol| symbol.x >= xmin && symbol.x <= xmax && symbol.y >= ymin && symbol.y <= ymax
    );
}



#[allow(dead_code)]
pub fn day3() {
    day3_1();
}

pub fn day3_1() {
    let input = get_input_or_panic("3-1");
    let field = parse_field(input);
    let n_numbers = field.numbers.len();
    let n_symbols = field.symbols.len();

    println!("First Symbol: {:?}", field.symbols.clone().first().unwrap());
    println!("First Number: {:?}", field.numbers.clone().first().unwrap());

    let valid_numbers : Vec<Number> = field.numbers
        .into_iter()
        .filter(|number| check_if_number_is_part_number(number, &field.symbols))
        .collect();
    let count_valid = valid_numbers.len();
    let sum : u32 = valid_numbers.into_iter().map(|number| number.value).sum();

    assert_eq!(sum, 556367);
    println!("Total amount of Symbols and Numbers: {} / {}", n_symbols, n_numbers);
    println!("Sum of {} valid part numbers: {}", count_valid, sum);
}