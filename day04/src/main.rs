use std::io::{BufReader,BufRead};
use std::fs::File;

struct Elf {
    start_index: u32,
    end_index: u32,
}

impl Elf {
    fn wholly_contains_other_elf(&self, other_elf: &Elf) -> bool {
        self.start_index <= other_elf.start_index && self.end_index >= other_elf.end_index
    }

    fn overlaps_other_elf(&self, other_elf: &Elf) -> bool {
        self.start_index <= other_elf.end_index && other_elf.start_index <= self.end_index
    }
}

struct ElfPair {
    first_elf: Elf,
    second_elf: Elf,
}

impl ElfPair {
    fn pairs_wholly_overlap(&self) -> bool {
        self.first_elf.wholly_contains_other_elf(&self.second_elf) || self.second_elf.wholly_contains_other_elf(&self.first_elf)
    }

    fn pairs_overlap(&self) -> bool {
        self.first_elf.overlaps_other_elf(&self.second_elf)
    }
}

fn main() {
    println!("Part a");
    println!("Example result: {}, expected: 2", solve_a("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_a("./inputs/input.txt"));
    
    println!("\nPart b");
    println!("Example result: {}, expected: 4", solve_b("./inputs/example.txt"));
    println!("Puzzle input: {}", solve_b("./inputs/input.txt"));
}

fn solve_a(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let mut overlap_count = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let elf_pair = get_elves(line);

        if elf_pair.pairs_wholly_overlap() {
            overlap_count += 1;
        }
    }

    overlap_count
}

fn solve_b(filename: &str) -> u32 {
    let file = File::open(filename).unwrap();
    let mut overlap_count = 0;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        let elf_pair = get_elves(line);

        if elf_pair.pairs_overlap() {
            overlap_count += 1;
        }
    }

    overlap_count
}

fn get_elves(line: String) -> ElfPair {
    let elves: Vec<Vec<u32>> = line.split(',').map(|elf| elf.split('-').map(|border| border.parse().unwrap()).collect()).collect();

    ElfPair {
        first_elf: Elf {
            start_index: elves[0][0],
            end_index: elves[0][1],
        },
        second_elf: Elf {
            start_index: elves[1][0],
            end_index: elves[1][1],
        },
    }
}