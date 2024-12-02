use crate::tools::{get_input_or_panic, input_with_lines_with_spaces_and_numbers_to_vec_of_vec_of_numbers};
type Report = Vec<i64>;

pub fn day2() {
    day2_1();
    day2_2();
}

fn day2_1() {
    let input = get_input_or_panic("2-1", 2024);
    let reports : Vec<Report> = input_with_lines_with_spaces_and_numbers_to_vec_of_vec_of_numbers(input.as_str()).unwrap();

    let mut safe = 0;
    for report in reports {
        if is_safe_report(&report) {
            safe += 1;
        }
    }

    assert_eq!(safe, 379);
    println!("Number of safe reports: {safe}");
}

fn is_safe_report(report : &Report) -> bool {
    let differences: Report = report
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    //print!("Report: {:?} ", report);

    if differences.iter().any(|d| *d == 0 || d.abs() > 3) {
        //println!(" -> unsafe, diff");
        false
    }
    else if differences
        .windows(2)
        .any(|w| w[0].is_positive() != w[1].is_positive()) {
        //println!(" -> unsafe, sign");
        false
    }
    else {
        //println!(" -> safe");
        true
    }
}

pub fn is_safe_report_with_dampener(report : &Report) -> bool {
    if !is_safe_report(report) {
        let subreports = generate_dampened_reports(report);
        return subreports.iter().any(is_safe_report);
    }
    else {
        true
    }
}

fn generate_dampened_reports(report: &Report) -> Vec<Report> {
    let max = report.len();
    let mut rv : Vec<Report> = Vec::new();
    for i in 0..max {
        let mut subreport : Report = Vec::new();
        let mut iter = report.iter();

        for j in 0..max {
            let next = iter.next().unwrap();
            if i != j {
                subreport.push(*next);
            }
        }

        rv.push(subreport);
    }

    rv
}

fn day2_2() {
    let _input = get_input_or_panic("2-1", 2024);

    let input = get_input_or_panic("2-1", 2024);
    let reports : Vec<Report> = input_with_lines_with_spaces_and_numbers_to_vec_of_vec_of_numbers(input.as_str()).unwrap();

    let mut safe = 0;
    for report in reports {
        if is_safe_report_with_dampener(&report) {
            safe += 1;
        }
    }

    assert_eq!(safe, 430);
    println!("Number of safe reports w/ dampener: {safe}");
}

