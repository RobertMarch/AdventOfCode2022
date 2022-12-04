use crate::days::day::Day;

#[derive(Copy, Clone)]
struct Elf {
    start_index: u32,
    end_index: u32,
}

impl Elf {
    fn get_from_input(input: &str) -> Elf {
        let elf_range: Vec<u32> = input.split('-').map(|border| border.parse().unwrap()).collect();

        Elf {
            start_index: elf_range[0],
            end_index: elf_range[1],
        }
    }

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
    fn get_from_line(line: &str) -> ElfPair {
        let elves: Vec<Elf> = line.split(',').map(|elf| Elf::get_from_input(elf)).collect();

        ElfPair {
            first_elf: elves[0],
            second_elf: elves[1],
        }
    }

    fn pairs_wholly_overlap(&self) -> bool {
        self.first_elf.wholly_contains_other_elf(&self.second_elf) || self.second_elf.wholly_contains_other_elf(&self.first_elf)
    }

    fn pairs_overlap(&self) -> bool {
        self.first_elf.overlaps_other_elf(&self.second_elf)
    }
}

pub struct Day04 {}

impl Day for Day04 {
    fn solve_a(&self, file: &String) -> String {
        let mut overlap_count = 0;

        for line in file.lines() {
            let elf_pair = ElfPair::get_from_line(line);

            if elf_pair.pairs_wholly_overlap() {
                overlap_count += 1;
            }
        }

        overlap_count.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut overlap_count = 0;

        for line in file.lines() {
            let elf_pair = ElfPair::get_from_line(line);

            if elf_pair.pairs_overlap() {
                overlap_count += 1;
            }
        }

        overlap_count.to_string()
    }
}
