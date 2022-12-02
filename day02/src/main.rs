use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
    println!("Part a");
    println!("Example result: {}, expected: 15", solve_a("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_a("./inputs/input.txt"));
    
    println!("\nPart b");
    println!("Example result: {}, expected: 12", solve_b("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_b("./inputs/input.txt"));
}

fn solve_a(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut total_score = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let other_elf_action = get_action_value_for_input(line.chars().nth(0).unwrap());
        let my_action = get_action_value_for_input(line.chars().nth(2).unwrap());
        
        total_score += ((my_action - other_elf_action + 4) % 3) * 3 + my_action;
    }

    total_score
}

fn solve_b(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let mut total_score = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let other_elf_action = get_action_value_for_input(line.chars().nth(0).unwrap());
        let target_result = get_action_value_for_input(line.chars().nth(2).unwrap()) - 2;
        let my_action = modulo_three(other_elf_action + target_result - 1) + 1;

        total_score += (target_result + 1) * 3 + my_action;
    }

    total_score
}

fn get_action_value_for_input(letter: char) -> i32 {
    match letter {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Unexpected character: {}", letter)
    }
}

fn modulo_three(value: i32) -> i32 {
    (value + 3) % 3
}
