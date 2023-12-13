use crate::tools::get_input_or_panic;
use std::cmp::min;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    scratched_numbers: Vec<u32>,
}

#[derive(Debug)]
struct CardStack {
    amount: u32,
    card: Card,
}

fn parse_numbers_from_str(input: &str) -> Vec<u32> {
    return input
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
}

fn parse_card(line: &str) -> Card {
    // Input: "Card   3: 96 46 60 19 82 25 41 29 38 94 | 43 82 86 74 16 15 92 46 32  3 17 30 42 98 60 12 96 38 19 35  6 29 72 25 62"
    //let number = &line[5..8];
    //let number : u32 = number.trim().parse().unwrap();

    let mut parts = (&line[10..]).split("|");
    assert_eq!(parts.clone().count(), 2);

    let winning_numbers = parse_numbers_from_str(parts.next().unwrap());
    let scratched_numbers = parse_numbers_from_str(parts.next().unwrap());

    Card {
        winning_numbers,
        scratched_numbers,
    }
}

fn card_value(card: &Card) -> u32 {
    let matches = card_n_winning_numbers(card);

    if matches > 0 {
        return u32::pow(2, matches - 1);
    } else {
        return 0;
    };
}

fn card_n_winning_numbers(card: &Card) -> u32 {
    let mut matches = 0;
    for number in card.scratched_numbers.iter() {
        if card.winning_numbers.contains(&number) {
            matches += 1;
        }
    }

    return matches;
}

fn card_stack_from_card(card: &Card) -> CardStack {
    let card = card.clone();
    return CardStack { card, amount: 1 };
}

fn increase_card_amounts(
    card_stack: &mut Vec<CardStack>,
    start_index: usize,
    amount: usize,
    amount_to_increase_by: u32,
) {
    for i in start_index..min(start_index + amount, card_stack.len()) {
        card_stack[i].amount += amount_to_increase_by;
    }
}

fn find_amount_of_cards(cards: &Vec<Card>) -> u32 {
    let mut stack: Vec<CardStack> = cards.iter().map(card_stack_from_card).collect();

    for index in 0..stack.len() {
        let card_stack = &stack[index];
        let card_value = card_n_winning_numbers(&card_stack.card);
        let card_amount = card_stack.amount;
        increase_card_amounts(&mut stack, index + 1, card_value as usize, card_amount);
    }

    return stack.into_iter().map(|cs| cs.amount).sum();
}

#[allow(dead_code)]
pub fn day4() {
    day4_1();
    day4_2();
}

fn day4_1() {
    let input = get_input_or_panic("4-1");
    let lines: Vec<&str> = input.lines().collect();
    let cards: Vec<Card> = lines.into_iter().map(parse_card).collect();
    let card_values: Vec<u32> = cards.iter().map(card_value).collect();
    let sum: u32 = card_values.iter().sum();

    assert_eq!(sum, 24848);
    println!("Sum of card values: {}", sum);
}

fn day4_2() {
    let input = get_input_or_panic("4-1");
    let lines: Vec<&str> = input.lines().collect();
    let cards: Vec<Card> = lines.into_iter().map(parse_card).collect();
    let sum = find_amount_of_cards(&cards);

    assert_eq!(sum, 7258152);
    println!("Total amount of cards: {}", sum);
}
