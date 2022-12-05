use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn solve_a(&self, file: &String) -> String {
        let (initial, moves) = file.split_once("\n\n").unwrap();

        let mut cargo_stacks: HashMap<usize, Vec<char>> = (1..10).map(|n| (n, vec![])).collect();

        for line in initial.lines() {
            if line.chars().nth(1).unwrap() == '1' {
                break;
            }

            for (i, chr) in line.chars().enumerate() {
                let column_number = (i + 3) / 4;
                if (i + 3) % 4 == 0 && chr != ' ' {
                    cargo_stacks.get_mut(&column_number).unwrap().splice(0..0, std::iter::once(chr));
                }
            }
        }

        for line in moves.lines() {
            let parts: Vec<&str> = line.split(" ").collect();
            let move_count: usize = parts[1].parse().expect("Unexpected number of times");
            let source: usize = parts[3].parse().expect("Unexpected stack number");
            let target: usize = parts[5].parse().expect("Unexpected stack number");

            for _n in 1..move_count+1 {
                let popped_val = cargo_stacks.get_mut(&source).unwrap().pop().unwrap();
                cargo_stacks.get_mut(&target).unwrap().push(popped_val);

            }

        }
        print_stacks(&cargo_stacks, 10);
        
        (1..10).map(|n| cargo_stacks.get_mut(&n).unwrap().pop().unwrap_or(' ')).collect()
    }
    
    fn solve_b(&self, _file: &String) -> String {
        String::from("Not yet implemented")
    }
}

fn print_stacks(cargo_stacks: &HashMap<usize, Vec<char>>, n_to_print: usize) {
    (1..n_to_print+1).for_each(|n| println!("{:?}", cargo_stacks.get(&n).unwrap()));
}
