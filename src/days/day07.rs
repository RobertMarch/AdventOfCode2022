use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day07 {}

impl Day for Day07 {
    fn solve_a(&self, file: &String) -> String {
        solve(file).values().filter(|size| size <= &&100000).sum::<usize>().to_string()
    }
    
    fn solve_b(&self, file: &String) -> String {
        let folder_sizes = solve(file);

        let space_needed = folder_sizes.get(&String::from("/")).unwrap() - 40000000;
        
        folder_sizes.values().filter(|size| size > &&space_needed).min().unwrap().to_string()
    }
}

fn solve(file: &String) -> HashMap<String, usize> {
    let mut current_address: Vec<String> = vec![String::from("/")];
    let mut folder_sizes: HashMap<String, usize> = HashMap::new();
    
    let commands = file.split("$ ");
    for command in commands {
        if command.trim().len() == 0 {
            continue;
        }

        if command.starts_with("ls") {
            handle_list(command, &current_address, &mut folder_sizes);
        } else {
            handle_cd(command.trim(), &mut current_address);
        }
    }

    folder_sizes
}

fn handle_list(command: &str, current_address: &Vec<String>, folder_sizes: &mut HashMap<String, usize>) {
    let lines = command.split("\n");
    for line in lines {
        if line.trim().len() == 0 || line.eq("ls") || line.starts_with("dir ") {
            continue;
        }

        let file_size: usize = line.split_once(" ").unwrap().0.parse().expect("Unexpected file size");

        for depth in 0..current_address.len() {
            let ancestor_folder = current_address[0..depth+1].join(";");

            *folder_sizes.entry(ancestor_folder).or_insert(0) += file_size;
        }
    }
}

fn handle_cd(command: &str, current_address: &mut Vec<String>) -> () {
  match command {
    "cd .." => {
        current_address.pop();
    }
    "cd /" => {
        current_address.splice(0..current_address.len(), std::iter::once(String::from("/")));
    }
    _ => {
        current_address.push(command.split_at(2).1.to_string())
    }
  };
}