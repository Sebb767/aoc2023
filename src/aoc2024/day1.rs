use std::collections::HashMap;
use std::hash::Hash;
use crate::tools::get_input_or_panic;

pub fn day1() {
    day1_1();
    day1_2();
}

fn read_input() -> (Vec<u64>, Vec<u64>) {
    let input = get_input_or_panic("1-1", 2024);
    let lines: Vec<&str> = input.lines().collect();
    let count = lines.len();

    let mut left: Vec<u64> = Vec::with_capacity(count);
    let mut right: Vec<u64> = Vec::with_capacity(count);

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

    return (left, right);
}

fn day1_1() {
    let (left, right) = read_input();

    let mut ileft = left.into_iter();
    let mut iright = right.into_iter();
    let mut result = 0u64;
    while let Some(nleft) = ileft.next() {
        let nright = iright.next().unwrap();
        result += u64::abs_diff(nright, nleft);
    }

    println!("Sum of differences of numbers: {result}");
}

fn list_to_occurrence_map<T>(inp : &Vec<T>) -> HashMap<T, u64> where T : Hash + Eq + Copy {
    let mut map: HashMap<T, u64> = HashMap::new();

    for num in inp.into_iter() {
        if let Some(kref) = map.get_mut(&num) {
            *kref += 1;
        }
        else {
            map.insert(*num, 1);
        }
    }

    map
}

fn day1_2() {
    let (left, right) = read_input();
    let lmap = list_to_occurrence_map(&left);
    let rmap = list_to_occurrence_map(&right);

    let mut result = 0u64;
    for key in lmap.keys() {
        let amount_right = *lmap.get(key).unwrap();
        if let Some(amount_left) = rmap.get(key) {
            result += *key * amount_right * *amount_left;
        }
    }

    println!("Difference score: {result}");
}