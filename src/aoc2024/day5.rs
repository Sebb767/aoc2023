use std::cmp::Ordering;
use std::collections::HashMap;
use crate::day::{Day, DayResult, ExpectedResults, YearDay};
use crate::ExpectedResult;

pub(crate) struct Day5;

type PageNumber = i128;

type PrintOrder = Vec<PageNumber>;

struct OrderRule {
    lower: PageNumber,
    upper: PageNumber,
}

struct PrintJobVerifier {
    rules_map: HashMap<PageNumber, Vec<PageNumber>>
}

impl Day for Day5 {
    fn part1(&self, input: String) -> Option<DayResult> {
        let (order_rules, print_jobs) = preprocess_input(&input)?;
        let verifier = PrintJobVerifier::create(order_rules);

        let result: DayResult = print_jobs.iter().map(|job| {
            match verifier.is_valid_print_job(job) {
                true => get_value_of_print_order(job),
                false => 0,
            }
        }).sum();

        Some(result)
    }

    fn part2(&self, input: String) -> Option<DayResult> {
        let (order_rules, print_jobs) = preprocess_input(&input)?;
        let verifier = PrintJobVerifier::create(order_rules);

        let result: DayResult = print_jobs.iter().map(|job| {
            match verifier.is_valid_print_job(job) {
                true => 0,
                false => {
                    let fixed = verifier.fix_print_job(job);
                    //dbg!(format!("Before: {:#?}", job));
                    //dbg!(format!("After : {:#?}", fixed));

                    get_value_of_print_order(&fixed)
                },
            }
        }).sum();

        Some(result)
    }

    fn get_expected_results(&self) -> ExpectedResults {
        ExpectedResult!(143, 4905, 123, 6204)
    }

    fn get_year_and_date(&self) -> YearDay {
        YearDay::y2024(5)
    }

    fn part1_result_description(&self) -> String {
        String::from("Sum of middle numbers of valid print reports")
    }

    fn part2_result_description(&self) -> String {
        String::from("Sum of middle numbers of invalid fixed reports")
    }
}

impl PrintJobVerifier {
    fn create(rules: Vec<OrderRule>) -> Self {
        let mut rules_map: HashMap<PageNumber, Vec<PageNumber>> = HashMap::new();

        for rule in rules {
            if let Some(v) = rules_map.get_mut(&rule.upper) {
                v.push(rule.lower);
            } else {
                rules_map.insert(rule.upper, vec![rule.lower]);
            }
        }

        Self {
            rules_map
        }
    }

    fn is_valid_print_job(&self, print_order: &PrintOrder) -> bool {
        for (idx, page) in print_order.iter().enumerate() {
            if let Some(lowers) = self.rules_map.get(page) {
                for lower in lowers {
                    if print_order[idx..].contains(lower) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn fix_print_job(&self, print_order: &PrintOrder) -> PrintOrder {
        let mut print_order = print_order.clone();

        print_order.sort_by(|a, b| {
            if let Some(rules) = self.rules_map.get(a) {
                if rules.contains(b) {
                    // b must be before a -> a < b
                    return Ordering::Greater;
                }
            }

            if let Some(rules) = self.rules_map.get(b) {
                if rules.contains(a) {
                    // a must be before b -> a > b
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });


        print_order
    }
}

fn preprocess_input(input: &String) -> Option<(Vec<OrderRule>, Vec<PrintOrder>)> {
    let results = input.split("\n\n");
    let results: Vec<&str> = results.collect();
    assert_eq!(results.len(), 2);

    if let [ rules_raw, orders_raw ] = results[0..2] {
        let rules: Vec<OrderRule> = rules_raw.lines().map(parse_rule).collect();
        let orders: Vec<PrintOrder> = orders_raw.lines().map(parse_print_job).collect();
        return Some((rules, orders));
    }
    unreachable!()
}

fn parse_rule(input: &str) -> OrderRule {
    let results = input.split("|");
    let results: Vec<&str> = results.collect();
    assert_eq!(results.len(), 2);
    OrderRule {
        lower: results.get(0).unwrap().parse().unwrap(),
        upper: results.get(1).unwrap().parse().unwrap(),
    }
}

fn parse_print_job(input: &str) -> PrintOrder {
    input.split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn get_value_of_print_order(print_order: &PrintOrder) -> PageNumber {
    let len = print_order.len();
    assert_eq!(len % 2, 1);
    *print_order.get(len / 2).unwrap()
}