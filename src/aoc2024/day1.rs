use crate::tools::get_input_or_panic;

pub fn day1() {
    day1_1();
    day1_2();
}

fn day1_1() {
    let input = get_input_or_panic("1-1", 2024);
    let lines : Vec<&str> = input.lines().collect();
    let count = lines.len();

    let mut left : Vec<u64> = Vec::with_capacity(count);
    let mut right : Vec<u64> = Vec::with_capacity(count);

    for line in lines.into_iter() {
        let numbers = line.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(numbers.len(), 2);
        let mut iter = numbers.into_iter();
        left.push(iter.next().unwrap().parse().unwrap());
        right.push(iter.next().unwrap().parse().unwrap());
    }

    left.sort();
    right.sort();
    assert_eq!(left.len(), right.len());

    let mut ileft = left.into_iter();
    let mut iright = right.into_iter();
    let mut result = 0u64;
    while let Some(nleft) = ileft.next() {
        let nright = iright.next().unwrap();
        result += u64::abs_diff(nright, nleft);
    }

    println!("Sum of differences of numbers: {result}");
}

fn day1_2() {
    let _input = get_input_or_panic("1-1", 2024);
}