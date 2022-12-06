use std::collections::HashSet;

use crate::days::day::Day;

pub struct Day06 {}

impl Day for Day06 {
    fn solve_a(&self, file: &String) -> String {
        find_marker_of_length(file, 4)
    }
    
    fn solve_b(&self, file: &String) -> String {
        find_marker_of_length(file, 14)
    }
}

fn find_marker_of_length(file: &String, target_length: usize) -> String {
    for i in (target_length-1)..file.len() {
        let char_set: HashSet<char> = file[i+1-target_length..i+1].chars().collect();

        if char_set.len() == target_length {
            return String::from((i+1).to_string())
        }
    }

    String::from("No solution found")
}
