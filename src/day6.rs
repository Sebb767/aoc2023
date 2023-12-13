use crate::tools::{get_input_or_panic, string_with_spaces_and_numbers_to_vec_of_numbers};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_races(input: String) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    assert!(times.starts_with("Time:       "));
    assert!(distances.starts_with("Distance:   "));
    assert_eq!(lines.next(), None);

    let times: Vec<i64> = string_with_spaces_and_numbers_to_vec_of_numbers(&times[12..]).unwrap();
    let distances: Vec<i64> =
        string_with_spaces_and_numbers_to_vec_of_numbers(&distances[12..]).unwrap();
    assert!(times.len() > 0);
    assert_eq!(times.len(), distances.len());

    let mut result = Vec::with_capacity(times.capacity());
    let mut times_iter = times.iter();
    let mut distances_iter = distances.iter();

    while let Some(time) = times_iter.next() {
        let distance = distances_iter.next().unwrap();
        result.push(Race {
            time: *time,
            distance: *distance,
        })
    }

    result
}

fn parse_race(input: String) -> Race {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let distances = lines.next().unwrap();

    assert!(times.starts_with("Time:       "));
    assert!(distances.starts_with("Distance:   "));
    assert_eq!(lines.next(), None);

    let time: i64 = times[12..].replace(" ", "").parse().unwrap();
    let distance: i64 = distances[12..].replace(" ", "").parse().unwrap();

    return Race { time, distance };
}

fn find_n_valid_strategies(race: Race) -> i64 {
    // 0 is never valid and neither is max len
    (1..race.time)
        .filter(is_button_time_winning_hof(&race))
        .count() as i64
}

fn find_n_valid_strategies_fast(race: Race) -> i64 {
    let mut min: Option<i64> = None;
    let mut max: Option<i64> = None;

    for possible_min in 1..race.time {
        if is_button_time_winning(&race, possible_min) {
            min = Some(possible_min);
            break;
        }
    }
    assert!(min.is_some());

    for possible_max in (min.unwrap()..race.time).rev() {
        if is_button_time_winning(&race, possible_max) {
            max = Some(possible_max);
            break;
        }
    }
    assert!(max.is_some());
    assert!(min <= max);

    return max.unwrap() - min.unwrap() + 1;
}

fn is_button_time_winning_hof(race: &Race) -> Box<dyn Fn(&i64) -> bool + '_> {
    Box::new(|inp| is_button_time_winning(race, *inp))
}

fn is_button_time_winning(race: &Race, button_time: i64) -> bool {
    assert!(race.time >= button_time);
    let drive_time = race.time - button_time;
    let dist = drive_time * button_time /* a.k.a. speed */;
    return dist > race.distance;
}

#[allow(dead_code)]
pub fn day6() {
    day6_1();
    day6_2();
}

fn day6_1() {
    let input = get_input_or_panic("6-1");
    let races = parse_races(input);
    let n_races = races.len();
    let n_winning_strategies: Vec<i64> = races.into_iter().map(find_n_valid_strategies).collect();
    let result: i64 = n_winning_strategies.iter().product();

    assert_eq!(result, 500346);
    println!("Product of possibilities for {n_races} races: {result}");
}

fn day6_2() {
    let input = get_input_or_panic("6-1");
    let race = parse_race(input);
    let result = find_n_valid_strategies_fast(race);

    assert_eq!(result, 42515755);
    println!("Possibilities for single race: {result}")
}
