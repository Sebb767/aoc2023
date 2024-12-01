use crate::tools::get_input_or_panic;
use std::cmp::max;

struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    draws: Vec<Draw>,
}

fn parse_draw(line: &str) -> Draw {
    // Input:
    // "2 green, 1 blue"
    // "1 red, 2 green, 4 blue"
    let mut red: i32 = 0;
    let mut green: i32 = 0;
    let mut blue: i32 = 0;

    let parts = line.split(',');
    assert!(parts.clone().count() > 0); // max three colours
    assert!(parts.clone().count() <= 3);

    for part in parts {
        // parts should now be for example ["5", "red"]
        let mut parts = part.trim().split(' ');
        assert_eq!(parts.clone().count(), 2);

        let value: i32 = parts.next().unwrap().parse().unwrap();
        let colour = parts.next().unwrap();

        match colour {
            "red" => red = value,
            "green" => green = value,
            "blue" => blue = value,
            _ => panic!("Unkown colour: {}!", colour),
        };
    }

    Draw { red, green, blue }
}

fn parse_line(line: &str) -> Game {
    // "Game 68: 1 red, 3 green; 1 blue; 2 green; 3 red, 1 blue; 1 green, 3 red, 2 blue"
    assert!(line.starts_with("Game "));
    let mut parts = line.split(": ");
    assert_eq!(parts.clone().count(), 2);
    let game_topic = parts.next().unwrap();
    let draws = parts.next().unwrap();

    // parse the game id
    // it always starts with "Game ", so we can simply cut the first 5 chars
    let id: i32 = game_topic[5..].parse().unwrap();

    // parse the draws
    let draws = draws.split("; ");
    assert!(draws.clone().count() > 0);
    let draws: Vec<Draw> = draws.map(parse_draw).collect();

    Game { id, draws }
}

fn is_valid_draw(input: &Draw, reference: &Draw) -> bool {
    input.blue <= reference.blue && input.red <= reference.red && input.green <= reference.green
}

fn is_valid_game(input: &Game, reference: &Draw) -> bool {
    return input.draws.iter().all(|d| is_valid_draw(d, reference));
}

#[allow(dead_code)]
pub fn day2() {
    day2_1();
    day2_2();
}

fn get_minimum_reference(input: &Game) -> Draw {
    let mut reference_draw = Draw {
        red: 0,
        blue: 0,
        green: 0,
    };

    for draw in input.draws.iter() {
        reference_draw.red = max(reference_draw.red, draw.red);
        reference_draw.green = max(reference_draw.green, draw.green);
        reference_draw.blue = max(reference_draw.blue, draw.blue);
    }

    reference_draw
}

fn draw_power(input: &Draw) -> i32 {
    input.blue * input.green * input.red
}

fn game_power(input: Game) -> i32 {
    draw_power(&get_minimum_reference(&input))
}

fn day2_1() {
    let refernce = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let input = get_input_or_panic("2-1", 2023);
    let lines: Vec<&str> = input.lines().collect();
    let games: Vec<Game> = lines.into_iter().map(parse_line).collect();
    let valid_games: Vec<Game> = games
        .into_iter()
        .filter(|g| is_valid_game(g, &refernce))
        .collect();
    let sum: i32 = valid_games.into_iter().map(|g| g.id).sum();

    assert_eq!(sum, 2720);
    println!("Sum of ids of valid games: {}", sum);
}

fn day2_2() {
    let input = get_input_or_panic("2-1", 2023);
    let lines: Vec<&str> = input.lines().collect();
    let games: Vec<Game> = lines.into_iter().map(parse_line).collect();
    let powers: Vec<i32> = games.into_iter().map(game_power).collect();
    let sum: i32 = powers.into_iter().sum();

    assert_eq!(sum, 71535);
    println!("Sum of powers of games: {}", sum);
}
