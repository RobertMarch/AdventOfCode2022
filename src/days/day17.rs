use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day17 {}

impl Day for Day17 {
    fn solve_a(&self, file: &String) -> String {
        get_height_after_rocks(file, 2022).to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let (repeat_rock_count, repeat_rock_height): (usize, usize) = get_repeat_info(file, 10000);

        let total_moves: usize = 1_000_000_000_000;
        let repeat_loops = total_moves / repeat_rock_count;
        let additional_rocks = total_moves % repeat_rock_count;

        let additional_height = get_height_after_rocks(file, additional_rocks);

        let result: usize = repeat_loops * repeat_rock_height + additional_height;

        result.to_string()
    }
}

fn get_height_after_rocks(input: &String, rock_count: usize) -> usize {
    let mut rock_positions: HashSet<Point> = get_rock_bed_positions();
    let rock_templates: Vec<Rock> = get_falling_rock_shapes();

    let gas_jet: HashMap<char, Point> = get_gas_jet_operations();
    let down_move: Point = Point::new(0, -1);

    let mut move_number: usize = 0;

    for rock_number in 0..rock_count {
        let mut new_rock: Rock = rock_templates[rock_number % 5].clone();
        let max_height: isize = get_max_height(&rock_positions);

        new_rock.add_vector(&Point::new(2, max_height + 4));

        loop {
            let next_jet = gas_jet
                .get(&input.chars().nth(move_number % input.len()).unwrap())
                .unwrap();
            move_number += 1;

            if new_rock.can_move(next_jet, &rock_positions) {
                new_rock.add_vector(&next_jet);
            }

            if new_rock.can_move(&down_move, &rock_positions) {
                new_rock.add_vector(&down_move);
            } else {
                for point in new_rock {
                    rock_positions.insert(point);
                }
                break;
            }
        }
    }

    get_max_height(&rock_positions) as usize
}

fn get_repeat_info(input: &String, max_total: usize) -> (usize, usize) {
    let mut rock_positions: HashSet<Point> = get_rock_bed_positions();
    let rock_templates: Vec<Rock> = get_falling_rock_shapes();

    let gas_jet: HashMap<char, Point> = get_gas_jet_operations();
    let down_move: Point = Point::new(0, -1);

    let mut move_number: usize = 0;
    let mut first_repeat_rock_number: usize = 0;
    let mut first_repeat_rock_height: usize = 0;

    for rock_number in 0..max_total {
        let mut new_rock: Rock = rock_templates[rock_number % 5].clone();
        let max_height: isize = get_max_height(&rock_positions);

        new_rock.add_vector(&Point::new(2, max_height + 4));

        loop {
            if move_number > 0 && move_number % input.len() == 0 {
                if first_repeat_rock_number == 0 {
                    first_repeat_rock_number = rock_number;
                    first_repeat_rock_height = max_height as usize;
                } else {
                    return (
                        rock_number - first_repeat_rock_number,
                        max_height as usize - first_repeat_rock_height,
                    );
                }
            }

            let next_jet = gas_jet
                .get(&input.chars().nth(move_number % input.len()).unwrap())
                .unwrap();
            move_number += 1;

            if new_rock.can_move(next_jet, &rock_positions) {
                new_rock.add_vector(&next_jet);
            }

            if new_rock.can_move(&down_move, &rock_positions) {
                new_rock.add_vector(&down_move);
            } else {
                for point in new_rock {
                    rock_positions.insert(point);
                }
                break;
            }
        }
    }

    panic!("Did not find repeating rocks")
}

fn get_rock_bed_positions() -> HashSet<Point> {
    HashSet::from([
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
        Point::new(4, 0),
        Point::new(5, 0),
        Point::new(6, 0),
    ])
}

fn get_falling_rock_shapes() -> Vec<Rock> {
    vec![
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
        ],
        vec![
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(1, 2),
        ],
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
        ],
        vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ],
        vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ],
    ]
}

fn get_gas_jet_operations() -> HashMap<char, Point> {
    HashMap::from([('>', Point::new(1, 0)), ('<', Point::new(-1, 0))])
}

fn get_max_height(rock_positions: &HashSet<Point>) -> isize {
    rock_positions
        .iter()
        .max_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y
}

fn _display_rock(rock: &HashSet<Point>, top_n_rows: isize) {
    let max_height: isize = rock.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    for y in (max_height - top_n_rows)..max_height + 1 {
        for x in 0..7 {
            if rock.contains(&Point::new(x, max_height - y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n")
    }
    print!("\n\n")
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

type Rock = Vec<Point>;

trait RockMethods {
    fn add_vector(&mut self, vector_to_add: &Point);
    fn can_move(&mut self, vector_to_add: &Point, rock_positions: &HashSet<Point>) -> bool;
}

impl RockMethods for Rock {
    fn add_vector(&mut self, vector_to_add: &Point) {
        for point in self {
            point.x += vector_to_add.x;
            point.y += vector_to_add.y;
        }
    }

    fn can_move(&mut self, vector_to_add: &Point, rock_positions: &HashSet<Point>) -> bool {
        let mut can_move = true;

        for point in self {
            let new_x = point.x + vector_to_add.x;
            let new_y = point.y + vector_to_add.y;

            if new_x < 0 || new_x > 6 || rock_positions.contains(&Point::new(new_x, new_y)) {
                can_move = false;
                break;
            }
        }

        can_move
    }
}
