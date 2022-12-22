use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn solve_a(&self, file: &String) -> String {
        let (board, path) = Board::parse(file);
        
        let mut initial_position = Position {
            location: Point::new(1, 1),
            direction: 0,
        };
        let move_to_start = Instruction {
            turn: None,
            distance: Some(1),
        };
        
        initial_position = board.get_new_location(&initial_position, &move_to_start);

        for instruction in path {
            initial_position = board.get_new_location(&initial_position, &instruction);
        }

        (initial_position.location.y * 1000 + initial_position.location.x * 4 + initial_position.direction).to_string()
    }
    
    fn solve_b(&self, _file: &String) -> String {
        String::from("Not yet implemented")
    }
}


#[derive(Debug)]
struct Position {
    location: Point,
    direction: isize,
}

#[derive(Debug)]
struct Instruction {
    turn: Option<isize>,
    distance: Option<usize>,
}

type Path = Vec<Instruction>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point {
            x,
            y,
        }
    }
    
    fn add(a: &Self, b: &Self) -> Self {
        Point::new(a.x + b.x, a.y + b.y)
    }
}

#[derive(Debug)]
struct Board {
    open_tiles: HashSet<Point>,
    solid_walls: HashSet<Point>,
    all_tiles: HashSet<Point>,
}

impl Board {
    fn parse(file: &str) -> (Self, Path) {
        let (map_string, path_string) = file.split_once("\n\n").unwrap();
        
        let mut open_tiles: HashSet<Point> = HashSet::new();
        let mut solid_walls: HashSet<Point> = HashSet::new();
        for (y, line) in map_string.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                let point = Point::new((x+1) as isize, (y+1) as isize);
                if value.eq(&'.') {
                    open_tiles.insert(point);
                }
                if value.eq(&'#') {
                    solid_walls.insert(point);
                }
            }
        }

        let all_tiles = open_tiles.union(&solid_walls).map(|p| p.clone()).collect();
        
        let board = Board {
            open_tiles,
            solid_walls,
            all_tiles,
        };
        
        let path_parts = path_string.replace("R", " R ").replace("L", " L ");
        
        let mut path: Path = vec![];
        
        for (i, part) in path_parts.split(" ").enumerate() {
            if i % 2 == 1 {
                let turn_value = if part.eq("R") { 1 } else { -1 };
                path.push(Instruction {
                    turn: Some(turn_value),
                    distance: None,
                });
            } else {
                let distance = part.parse::<usize>().expect("could not parse value");
                path.push(Instruction {
                    turn: None,
                    distance: Some(distance),
                });
            }
        }
        
        (board, path)
    }
    
    fn get_new_location(&self, current_position: &Position, instruction: &Instruction) -> Position {
        if instruction.turn.is_some() {
            return Position {
                location: current_position.location,
                direction: (current_position.direction + instruction.turn.unwrap() + 4) % 4,
            }
        } else {
            let directions: HashMap<isize, Point> = HashMap::from([
                (0, Point::new(1, 0)),
                (1, Point::new(0, 1)),
                (2, Point::new(-1, 0)),
                (3, Point::new(0, -1)),
            ]);
            let direction_vector = directions.get(&current_position.direction).unwrap();
            let mut new_point = current_position.location;
            let mut moved_distance = 0;

            let (min_x, max_x) = self.get_bounds_for_row(new_point.y);
            let (min_y, max_y) = self.get_bounds_for_col(new_point.x);

            while moved_distance < instruction.distance.unwrap() {
                let mut next_point = Point::add(&new_point, direction_vector);
                if direction_vector.x != 0 && (next_point.x < min_x || next_point.x > max_x) {
                    if direction_vector.x == 1 {
                        next_point = Point::new(min_x - 1, next_point.y);
                    } else {
                        next_point = Point::new(max_x + 1, next_point.y);
                    }
                }
                if direction_vector.y != 0 && (next_point.y < min_y || next_point.y > max_y) {
                    if direction_vector.y == 1 {
                        next_point = Point::new(next_point.x, min_y - 1);
                    } else {
                        next_point = Point::new(next_point.x, max_y + 1);
                    }
                }
                
                if self.solid_walls.contains(&next_point) {
                    break;
                } else {
                    new_point = next_point;

                    if self.open_tiles.contains(&next_point) {
                        moved_distance += 1;
                    }
                }
            }
            return Position {
                location: new_point,
                direction: current_position.direction,
            }
        }
    }

    fn get_bounds_for_row(&self, y: isize) -> (isize, isize) {
        (
            self.all_tiles.iter().filter_map(|p| if p.y == y { Some(p.x) } else { None }).min().unwrap(),
            self.all_tiles.iter().filter_map(|p| if p.y == y { Some(p.x) } else { None }).max().unwrap()
        )
    }

    fn get_bounds_for_col(&self, x: isize) -> (isize, isize) {
        (
            self.all_tiles.iter().filter_map(|p| if p.x == x { Some(p.y) } else { None }).min().unwrap(),
            self.all_tiles.iter().filter_map(|p| if p.x == x { Some(p.y) } else { None }).max().unwrap()
        )
    }
}