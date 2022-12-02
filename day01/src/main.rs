use std::fs::File;
use std::io::Read;
use std::path::Path;

const EXAMPLE_FILE_NAME: &str = "./inputs/example.txt";
const INPUT_FILE_NAME: &str = "./inputs/input.txt";

fn main() {
    println!("Part a:");
    println!("Example result: {}, Expected: {}", calculate_maximum_calories_n(EXAMPLE_FILE_NAME, 1), 24000);
    println!("User input result: {}", calculate_maximum_calories_n(INPUT_FILE_NAME, 1));
    println!("\nPart b:");
    println!("Example result: {}, Expected: {}", calculate_maximum_calories_n(EXAMPLE_FILE_NAME, 3), 45000);
    println!("User input result: {}", calculate_maximum_calories_n(INPUT_FILE_NAME, 3));
}

fn calculate_maximum_calories_n(file: &str, length: usize) -> u32 {
    // Why does this need to use n+1??????
    let mut maximum_calories = vec![0; length+1];

    let file_contents = read_file(file);

    let elves = file_contents.split("\n\n");

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

    maximum_calories.iter().sum()
}

fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file: {}:\n {}", display, why),
        Ok(_) => (), //println!("Successfully read file!"),
    }
    s
}
