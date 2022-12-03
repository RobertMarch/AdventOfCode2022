use std::collections::HashSet;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::ops::BitAnd;

fn main() {
    println!("Part a");
    println!("Example result: {}, expected: 157", solve_a("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_a("./inputs/input.txt"));
    
    println!("\nPart b");
    println!("Example result: {}, expected: 70", solve_b("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_b("./inputs/input.txt"));
}

fn solve_a(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let mut total_priority = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let (compartment1, compartment2) = line.split_at(line.len() / 2);

        let compartment_1_set: HashSet<char> = compartment1.chars().into_iter().collect();
        let compartment_2_set: HashSet<char> = compartment2.chars().into_iter().collect();

        let repeated_char: char = compartment_1_set.intersection(&compartment_2_set).into_iter().next().unwrap().clone();

        total_priority += convert_char_to_priority(repeated_char);
    }

    total_priority
}

fn solve_b(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let mut total_priority = 0;

    let mut current_group_items: HashSet<char> = HashSet::new();

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();

        let elf_char_set: HashSet<char> = line.chars().into_iter().collect();

        match i % 3 {
            0 => {
                current_group_items = elf_char_set;
            }
            1 => {
                current_group_items = current_group_items.bitand(&elf_char_set);
            }
            2 => {
                let common_char: char = current_group_items.intersection(&elf_char_set).into_iter().next().unwrap().clone();
                total_priority += convert_char_to_priority(common_char);
            }
            _ => {
                println!("Unexpected remainder: {}", i)
            }
        }
    }

    total_priority
}

fn convert_char_to_priority(character: char) -> u32 {
    let char_unicode = character as u32;

    if char_unicode >= 65 && char_unicode <= 90 {
        return char_unicode - 65 + 27;
    }

    char_unicode - 96
}
