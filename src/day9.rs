use crate::tools::{get_input_or_panic, string_with_spaces_and_numbers_to_vec_of_numbers};
use std::ops::Sub;

type SensorValue = i64;
type SensorValueList = Vec<SensorValue>;

fn calculate_differences<T>(input: &Vec<T>) -> Vec<T>
where
    T: Sub<Output = T>,
    T: Copy,
{
    assert!(input.len() >= 2);
    let mut result = Vec::with_capacity(input.len() - 1);
    let mut iter = input.iter();
    let mut previous = iter.next().unwrap();

    for current in iter {
        result.push(*current - *previous);
        previous = current;
    }

    result
}

fn extrapolate_next_sensor_value(input: &SensorValueList) -> SensorValue {
    if input.iter().all(|value| *value == 0) {
        return 0;
    }

    let next = calculate_differences(input);
    /*
    The structure is like this:
    _ A X  <-- input
     _ B   <-- next
    So we get X-A=B, which we reformulate to X=A+B
    */
    let a = input.last().unwrap();
    let b = extrapolate_next_sensor_value(&next);
    *a + b
}

fn extrapolate_previous_sensor_value(input: &SensorValueList) -> SensorValue {
    if input.iter().all(|value| *value == 0) {
        return 0;
    }

    let next = calculate_differences(input);
    /*
    The structure is like this:
    X A _ <-- input
     B _  <-- next
    So we get A-X=B, which we reformulate to X=A-B
    */
    let a = input.first().unwrap();
    let b = extrapolate_previous_sensor_value(&next);
    *a - b
}

pub fn day9() {
    day9_1();
    day9_2();
}

fn day9_1() {
    let input = get_input_or_panic("9-1");
    let sensor_values = input
        .lines()
        .map(string_with_spaces_and_numbers_to_vec_of_numbers)
        .collect::<Result<Vec<SensorValueList>, _>>()
        .expect("Failed to read sensor values!");
    let result: SensorValue = sensor_values
        .iter()
        .map(extrapolate_next_sensor_value)
        .sum();

    //assert_eq!(result, 1);
    println!("Sum of extrapolated sensor values: {result}");
}

fn day9_2() {
    let input = get_input_or_panic("9-1");
    let sensor_values = input
        .lines()
        .map(string_with_spaces_and_numbers_to_vec_of_numbers)
        .collect::<Result<Vec<SensorValueList>, _>>()
        .expect("Failed to read sensor values!");
    let result: SensorValue = sensor_values
        .iter()
        .map(extrapolate_previous_sensor_value)
        .sum();

    assert_eq!(result, 971);
    println!("Sum of previous extrapolated sensor values: {result}");
}
