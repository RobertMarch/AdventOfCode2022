use crate::days::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn solve_a(&self, file: &String) -> String {
        calculate_max_n_calories(file, 1)
    }

    fn solve_b(&self, file: &String) -> String {
        calculate_max_n_calories(file, 3)
    }
}

fn calculate_max_n_calories(file: &String, num_elf_calories_to_sum: usize) -> String {
    let mut maximum_calories = vec![0; num_elf_calories_to_sum + 1];
    
    let elves = file.split("\n\n");
    
    for elf in elves {
        let elf_items = elf.split("\n");
        
        let mut elf_calories = 0;
        for elf_item in elf_items {
            let item_calories: u32 = elf_item.parse().expect("Not a number!");
            elf_calories += item_calories;
        }
        
        maximum_calories[0] = elf_calories;
        maximum_calories.sort();
        maximum_calories[0] = 0;
    }
    
    String::from(maximum_calories.iter().sum::<u32>().to_string())
}
