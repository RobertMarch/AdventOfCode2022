use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day23 {}

impl Day for Day23 {
    fn solve_a(&self, file: &String) -> String {
        let (empty_tile_count, _) = solve(file, 10);

        empty_tile_count.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let max_rounds = 10000;
        let (_, rounds) = solve(file, max_rounds);

        if rounds == max_rounds {
            return format!("Did not stabilise in {rounds} rounds");
        }

        rounds.to_string()
    }
}

fn solve(file: &String, max_rounds: usize) -> (isize, usize) {
    let (mut elves, mut elf_locations) = parse(file);

    for round in 0..max_rounds {
        let mut is_complete = true;

        let mut position_updates: HashMap<Point, usize> = HashMap::new();
        let mut duplicate_points: HashSet<Point> = HashSet::new();

        for elf in elves.iter() {
            if should_move(&elf.1, &elf_locations) {
                is_complete = false;

                let next_location = get_next_location(&elf.1, &round, &elf_locations);

                if next_location.is_none() || duplicate_points.contains(&next_location.unwrap()) {
                    continue;
                }
                if position_updates.contains_key(&next_location.unwrap()) {
                    position_updates.remove(&next_location.unwrap());
                    duplicate_points.insert(next_location.unwrap());
                    continue;
                }
                
                position_updates.insert(next_location.unwrap(), elf.0.clone());
            }
        }

        for (new_point, elf_id) in position_updates.iter() {
            let old_location = elves.remove(elf_id);
            elf_locations.remove(&old_location.unwrap());

            elves.insert(elf_id.clone(), new_point.clone());
            elf_locations.insert(new_point.clone());
        }

        if is_complete {
            return (0, round + 1);
        }
    }

    (get_empty_tiles(&elf_locations), max_rounds)
}

fn should_move(elf: &Point, elf_locations: &HashSet<Point>) -> bool {
    let mut has_neighbour = false;
    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) && elf_locations.contains(&Point::new(elf.x + i, elf.y + j)) {
                has_neighbour = true
            }
        }
    }

    has_neighbour
}

fn get_next_location(elf: &Point, round: &usize, elf_locations: &HashSet<Point>) -> Option<Point> {
    for move_option in 0..4 {
        let moves = match (round + move_option) % 4 {
            0 => vec![
                Point::new(elf.x - 1, elf.y - 1),
                Point::new(elf.x, elf.y - 1),
                Point::new(elf.x + 1, elf.y - 1),
            ],
            1 => vec![
                Point::new(elf.x - 1, elf.y + 1),
                Point::new(elf.x, elf.y + 1),
                Point::new(elf.x + 1, elf.y + 1),
            ],
            2 => vec![
                Point::new(elf.x - 1, elf.y - 1),
                Point::new(elf.x - 1, elf.y),
                Point::new(elf.x - 1, elf.y + 1),
            ],
            3 => vec![
                Point::new(elf.x + 1, elf.y - 1),
                Point::new(elf.x + 1, elf.y),
                Point::new(elf.x + 1, elf.y + 1),
            ],
            _ => panic!("Unexpected value"),
        };

        if !moves.iter().any(|p| elf_locations.contains(p)) {
            return Some(moves[1]);
        }
    }

    None
}

fn get_empty_tiles(elf_locations: &HashSet<Point>) -> isize {
    let min_x = elf_locations.iter().map(|p| p.x).min().unwrap();
    let max_x = elf_locations.iter().map(|p| p.x).max().unwrap();
    let min_y = elf_locations.iter().map(|p| p.y).min().unwrap();
    let max_y = elf_locations.iter().map(|p| p.y).max().unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - elf_locations.len() as isize
}

fn _print_board(elf_locations: &HashSet<Point>) {
    let min_x = elf_locations.iter().map(|p| p.x).min().unwrap();
    let max_x = elf_locations.iter().map(|p| p.x).max().unwrap();
    let min_y = elf_locations.iter().map(|p| p.y).min().unwrap();
    let max_y = elf_locations.iter().map(|p| p.y).max().unwrap();

    println!("\nCurrent board state:");
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if elf_locations.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("End of board\n");
}

fn parse(file: &String) -> (Elves, HashSet<Point>) {
    let mut elves: Elves = HashMap::new();
    let mut elf_locations: HashSet<Point> = HashSet::new();

    for (i, line) in file.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.eq(&'#') {
                elves.insert(100 * i + j, Point::new_usize(j, i));
                elf_locations.insert(Point::new_usize(j, i));
            }
        }
    }

    (elves, elf_locations)
}

type Elves = HashMap<usize, Point>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn new_usize(x: usize, y: usize) -> Self {
        Point { x: x as isize, y: y as isize }
    }
}
