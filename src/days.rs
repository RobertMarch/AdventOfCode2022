use std::fs::{read_to_string};

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
// Insert other day modules above

pub fn run_day(day: u8) -> Option<u8> {
    let (example_input, puzzle_input) = get_inputs(day);

    println!("Day {} part a", day);
    println!("Example input result: {}", get_day(day)?.solve_a(&example_input));
    println!("Puzzle input result: {}", get_day(day)?.solve_a(&puzzle_input));
    
    println!("\nDay {} part b", day);
    println!("Example result: {}", get_day(day)?.solve_b(&example_input));
    println!("Puzzle input result: {}", get_day(day)?.solve_b(&puzzle_input));
    
    Some(1)
}

fn get_inputs(day: u8) -> (String, String) {
    let example_filename = format!("./inputs/{:02}_example.txt", day);
    let input_filename = format!("./inputs/{:02}_input.txt", day);

    (get_file_as_string(example_filename), get_file_as_string(input_filename))
}

fn get_file_as_string(filename: String) -> String {
    read_to_string(filename).unwrap().parse().expect("Error reading file")
}

fn get_day(day: u8) -> Option<Box<dyn day::Day>> {
    match day {
        1 => Some(Box::new(day01::Day01{})),
        2 => Some(Box::new(day02::Day02{})),
        3 => Some(Box::new(day03::Day03{})),
        4 => Some(Box::new(day04::Day04{})),
        5 => Some(Box::new(day05::Day05{})),
        6 => Some(Box::new(day06::Day06{})),
        // Insert other day mappings above
        _ => panic!("Unknown day found"),
    }
}