use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day17 {}

impl Day for Day17 {
    fn solve_a(&self, file: &String) -> String {
        let mut rock_positions: HashSet<Point> = HashSet::from([
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(5, 0),
            Point::new(6, 0),
        ]);
        let rock_templates: Vec<Rock> = vec![
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
        ];
        let gas_jet: HashMap<char, Point> =
            HashMap::from([('>', Point::new(1, 0)), ('<', Point::new(-1, 0))]);
        let down_move: Point = Point::new(0, -1);

        let mut move_number: usize = 0;

        for rock_number in 0..2022 {
            let mut new_rock: Rock = rock_templates[rock_number % 5].clone();
            let max_height: isize = rock_positions
                .iter()
                .max_by(|a, b| a.y.cmp(&b.y))
                .unwrap()
                .y;

            new_rock.add_vector(&Point::new(2, max_height + 4));

            loop {
                let next_jet = gas_jet
                    .get(&file.chars().nth(move_number % file.len()).unwrap())
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

        rock_positions
            .iter()
            .max_by(|a, b| a.y.cmp(&b.y))
            .unwrap()
            .y
            .to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        println!("{}", file.len());
        String::from("Not yet implemented")
    }
}

fn _display_rock(rock: &HashSet<Point>) {
    let max_height: isize = rock.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    for y in 0..max_height + 1 {
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

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}
