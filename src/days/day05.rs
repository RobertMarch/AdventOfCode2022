use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn solve_a(&self, file: &String) -> String {
        solve(file, true)
    }
    
    fn solve_b(&self, file: &String) -> String {
        solve(file, false)
    }
}

fn solve(file: &String, reverse_order_in_move: bool) -> String {
    let (initial, moves) = file.split_once("\n\n").unwrap();

    let mut cargo_stacks: HashMap<usize, Vec<char>> = (1..10).map(|n| (n, vec![])).collect();

    for line in initial.lines() {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }

        for (i, chr) in line.chars().enumerate() {
            let column_number = (i + 3) / 4;
            if (i + 3) % 4 == 0 && chr != ' ' {
                cargo_stacks.get_mut(&column_number).unwrap().push(chr);
            }
        }
    }

    for line in moves.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let move_count: usize = parts[1].parse().expect("Unexpected number of times");
        let source: usize = parts[3].parse().expect("Unexpected stack number");
        let target: usize = parts[5].parse().expect("Unexpected stack number");

        let mut removed_elements: Vec<char> = cargo_stacks.get_mut(&source).unwrap().splice(0..move_count, std::iter::empty()).collect();

        if reverse_order_in_move {
            removed_elements.reverse();
        }
        
        cargo_stacks.get_mut(&target).unwrap().splice(0..0, removed_elements);
    }
    
    (1..10).map(|n| {
        let stack = cargo_stacks.get_mut(&n).unwrap();
        if stack.len() > 0 {
            return stack.splice(0..1, std::iter::empty()).collect::<String>();
        }
        String::from(' ')
    }).collect()
}
