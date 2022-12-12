use crate::days::day::Day;

use std::collections::HashSet;
use std::ops::BitAnd;

pub struct Day03 {}

impl Day for Day03 {
    fn solve_a(&self, file: &String) -> String {
        let mut total_priority = 0;

        for line in file.lines() {
            let (compartment1, compartment2) = line.split_at(line.len() / 2);

            let compartment_1_set: HashSet<char> = compartment1.chars().into_iter().collect();
            let compartment_2_set: HashSet<char> = compartment2.chars().into_iter().collect();

            let repeated_char: char = compartment_1_set
                .intersection(&compartment_2_set)
                .into_iter()
                .next()
                .unwrap()
                .clone();

            total_priority += convert_char_to_priority(repeated_char);
        }

        total_priority.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut total_priority = 0;

        let mut current_group_items: HashSet<char> = HashSet::new();

        for (i, line) in file.lines().enumerate() {
            let elf_char_set: HashSet<char> = line.chars().into_iter().collect();

            match i % 3 {
                0 => {
                    current_group_items = elf_char_set;
                }
                1 => {
                    current_group_items = current_group_items.bitand(&elf_char_set);
                }
                2 => {
                    let common_char: char = current_group_items
                        .intersection(&elf_char_set)
                        .into_iter()
                        .next()
                        .unwrap()
                        .clone();
                    total_priority += convert_char_to_priority(common_char);
                }
                _ => {
                    println!("Unexpected remainder: {}", i)
                }
            }
        }

        total_priority.to_string()
    }
}

fn convert_char_to_priority(character: char) -> u32 {
    let char_unicode = character as u32;

    if char_unicode >= 65 && char_unicode <= 90 {
        return char_unicode - 65 + 27;
    }

    char_unicode - 96
}
