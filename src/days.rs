use std::fs::read_to_string;
use std::time::Instant;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
// Insert other day modules above

pub fn run_day(day: u8, part: String, input: String) -> Option<u8> {
    let (example_input, puzzle_input) = get_inputs(day);

    if part.eq("a") || part.eq("both") {
        println!("Day {} part a", day);

        if input.eq("example") || input.eq("both") {
            println!(
                "Example input result: {}",
                get_day(day)?.solve_a(&example_input)
            );
        }

        if input.eq("puzzle") || input.eq("both") {
            let now = Instant::now();
            println!(
                "Puzzle input result: {}",
                get_day(day)?.solve_a(&puzzle_input)
            );
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        }
    }

    if part.eq("b") || part.eq("both") {
        println!("\nDay {} part b", day);

        if input.eq("example") || input.eq("both") {
            println!("Example result: {}", get_day(day)?.solve_b(&example_input));
        }

        if input.eq("puzzle") || input.eq("both") {
            let now = Instant::now();
            println!(
                "Puzzle input result: {}",
                get_day(day)?.solve_b(&puzzle_input)
            );
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
        }
    }

    Some(1)
}

fn get_inputs(day: u8) -> (String, String) {
    let example_filename = format!("./inputs/{:02}_example.txt", day);
    let input_filename = format!("./inputs/{:02}_input.txt", day);

    (
        get_file_as_string(example_filename),
        get_file_as_string(input_filename),
    )
}

fn get_file_as_string(filename: String) -> String {
    read_to_string(filename)
        .unwrap()
        .parse()
        .expect("Error reading file")
}

fn get_day(day: u8) -> Option<Box<dyn day::Day>> {
    match day {
        1 => Some(Box::new(day01::Day01 {})),
        2 => Some(Box::new(day02::Day02 {})),
        3 => Some(Box::new(day03::Day03 {})),
        4 => Some(Box::new(day04::Day04 {})),
        5 => Some(Box::new(day05::Day05 {})),
        6 => Some(Box::new(day06::Day06 {})),
        7 => Some(Box::new(day07::Day07 {})),
        8 => Some(Box::new(day08::Day08 {})),
        9 => Some(Box::new(day09::Day09 {})),
        10 => Some(Box::new(day10::Day10 {})),
        11 => Some(Box::new(day11::Day11 {})),
        12 => Some(Box::new(day12::Day12 {})),
        13 => Some(Box::new(day13::Day13 {})),
        14 => Some(Box::new(day14::Day14 {})),
        15 => Some(Box::new(day15::Day15 {})),
        16 => Some(Box::new(day16::Day16 {})),
        17 => Some(Box::new(day17::Day17 {})),
        18 => Some(Box::new(day18::Day18 {})),
        19 => Some(Box::new(day19::Day19 {})),
        20 => Some(Box::new(day20::Day20 {})),
        21 => Some(Box::new(day21::Day21 {})),
        22 => Some(Box::new(day22::Day22 {})),
        23 => Some(Box::new(day23::Day23 {})),
        24 => Some(Box::new(day24::Day24 {})),
        25 => Some(Box::new(day25::Day25 {})),
        // Insert other day mappings above
        _ => panic!("Unknown day found"),
    }
}
