use crate::days::day::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn solve_a(&self, file: &String) -> String {
        let mut total_score = 0;
        
        for line in file.lines() {
            let other_elf_action = get_action_value_for_input(line.chars().nth(0).unwrap());
            let my_action = get_action_value_for_input(line.chars().nth(2).unwrap());
        
            total_score += ((my_action - other_elf_action + 4) % 3) * 3 + my_action;
        }
        
        total_score.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut total_score = 0;
        
        for line in file.lines() {
            let other_elf_action = get_action_value_for_input(line.chars().nth(0).unwrap());
            let target_result = get_action_value_for_input(line.chars().nth(2).unwrap()) - 2;
            let my_action = modulo_three(other_elf_action + target_result - 1) + 1;
            
            total_score += (target_result + 1) * 3 + my_action;
        }
        
        total_score.to_string()
    }
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
