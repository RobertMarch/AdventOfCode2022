use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn solve_a(&self, file: &String) -> String {
        solve(file, false)
    }

    fn solve_b(&self, file: &String) -> String {
        solve(file, true)
    }
}

fn solve(file: &String, part_two: bool) -> String {
    let (board, path) = Board::parse(file, part_two);

    let mut position = Position {
        location: Point::new(1, 1),
        direction: 0,
        face: 'A',
    };

    for instruction in path {
        position = board.get_new_location(&position, &instruction);
    }

    let face_origin = Board::get_face_origins(board.size)
        .get(&position.face)
        .unwrap()
        .clone();

    let final_point = Point::add(
        &Point::add(&position.location, &face_origin),
        &Point::new(-1, -1),
    );

    (final_point.y * 1000 + final_point.x * 4 + position.direction).to_string()
}

#[derive(Debug, Clone)]
struct Position {
    location: Point,
    direction: isize,
    face: char,
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
        Point { x, y }
    }

    fn add(a: &Self, b: &Self) -> Self {
        Point::new(a.x + b.x, a.y + b.y)
    }
}

type Face = HashSet<Point>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct FaceJoin {
    face_id: char,
    orientation: isize,
}

impl FaceJoin {
    fn new(face_id: char, orientation: isize) -> Self {
        FaceJoin {
            face_id,
            orientation,
        }
    }
}

type FaceJoins = HashMap<char, HashMap<isize, FaceJoin>>;

#[derive(Debug)]
struct Board {
    faces: HashMap<char, Face>,
    face_joins: FaceJoins,
    size: isize,
}

impl Board {
    fn parse(file: &str, construct_cube: bool) -> (Self, Path) {
        let (map_string, path_string) = file.split_once("\n\n").unwrap();

        let mut open_tiles: HashSet<Point> = HashSet::new();
        let mut solid_walls: HashSet<Point> = HashSet::new();
        for (y, line) in map_string.lines().enumerate() {
            for (x, value) in line.chars().enumerate() {
                let point = Point::new((x + 1) as isize, (y + 1) as isize);
                if value.eq(&'.') {
                    open_tiles.insert(point);
                }
                if value.eq(&'#') {
                    solid_walls.insert(point);
                }
            }
        }

        let size: isize = if (open_tiles.len() + solid_walls.len()) / 6 == 16 {
            4
        } else {
            50
        };

        let face_origins: HashMap<char, Point> = Board::get_face_origins(size);

        let mut faces: HashMap<char, Face> = HashMap::new();

        for (face_id, face_origin) in face_origins.iter() {
            let face_walls: Face = solid_walls
                .iter()
                .filter_map(|p| {
                    if p.x >= face_origin.x
                        && p.x < face_origin.x + size
                        && p.y >= face_origin.y
                        && p.y < face_origin.y + size
                    {
                        Some(Point::new(p.x - face_origin.x + 1, p.y - face_origin.y + 1))
                    } else {
                        None
                    }
                })
                .collect();

            faces.insert(face_id.clone(), face_walls);
        }

        let board = Board {
            faces,
            face_joins: Board::get_face_joins(size, construct_cube),
            size,
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

    fn get_face_origins(size: isize) -> HashMap<char, Point> {
        if size == 4 {
            return HashMap::from([
                ('A', Point::new(9, 1)),
                ('B', Point::new(1, 5)),
                ('C', Point::new(5, 5)),
                ('D', Point::new(9, 5)),
                ('E', Point::new(9, 9)),
                ('F', Point::new(13, 9)),
            ]);
        }
        HashMap::from([
            ('A', Point::new(51, 1)),
            ('B', Point::new(101, 1)),
            ('C', Point::new(51, 51)),
            ('D', Point::new(1, 101)),
            ('E', Point::new(51, 101)),
            ('F', Point::new(1, 151)),
        ])
    }

    fn get_face_joins(size: isize, construct_cube: bool) -> FaceJoins {
        if size == 4 {
            if !construct_cube {
                return HashMap::from([
                    (
                        'A',
                        HashMap::from([
                            (0, FaceJoin::new('A', 0)),
                            (1, FaceJoin::new('D', 0)),
                            (2, FaceJoin::new('A', 0)),
                            (3, FaceJoin::new('E', 0)),
                        ]),
                    ),
                    (
                        'B',
                        HashMap::from([
                            (0, FaceJoin::new('C', 0)),
                            (1, FaceJoin::new('B', 0)),
                            (2, FaceJoin::new('D', 0)),
                            (3, FaceJoin::new('B', 0)),
                        ]),
                    ),
                    (
                        'C',
                        HashMap::from([
                            (0, FaceJoin::new('D', 0)),
                            (1, FaceJoin::new('C', 0)),
                            (2, FaceJoin::new('B', 0)),
                            (3, FaceJoin::new('C', 0)),
                        ]),
                    ),
                    (
                        'D',
                        HashMap::from([
                            (0, FaceJoin::new('B', 0)),
                            (1, FaceJoin::new('E', 0)),
                            (2, FaceJoin::new('C', 0)),
                            (3, FaceJoin::new('A', 0)),
                        ]),
                    ),
                    (
                        'E',
                        HashMap::from([
                            (0, FaceJoin::new('F', 0)),
                            (1, FaceJoin::new('A', 0)),
                            (2, FaceJoin::new('F', 0)),
                            (3, FaceJoin::new('D', 0)),
                        ]),
                    ),
                    (
                        'F',
                        HashMap::from([
                            (0, FaceJoin::new('E', 0)),
                            (1, FaceJoin::new('F', 0)),
                            (2, FaceJoin::new('E', 0)),
                            (3, FaceJoin::new('F', 0)),
                        ]),
                    ),
                ]);
            }
            return HashMap::from([
                (
                    'A',
                    HashMap::from([
                        (0, FaceJoin::new('F', 2)),
                        (1, FaceJoin::new('D', 0)),
                        (2, FaceJoin::new('C', 1)),
                        (3, FaceJoin::new('B', 2)),
                    ]),
                ),
                (
                    'B',
                    HashMap::from([
                        (0, FaceJoin::new('C', 0)),
                        (1, FaceJoin::new('E', 2)),
                        (2, FaceJoin::new('F', 3)),
                        (3, FaceJoin::new('A', 2)),
                    ]),
                ),
                (
                    'C',
                    HashMap::from([
                        (0, FaceJoin::new('D', 0)),
                        (1, FaceJoin::new('E', 1)),
                        (2, FaceJoin::new('B', 0)),
                        (3, FaceJoin::new('A', 3)),
                    ]),
                ),
                (
                    'D',
                    HashMap::from([
                        (0, FaceJoin::new('F', 3)),
                        (1, FaceJoin::new('E', 0)),
                        (2, FaceJoin::new('C', 0)),
                        (3, FaceJoin::new('A', 0)),
                    ]),
                ),
                (
                    'E',
                    HashMap::from([
                        (0, FaceJoin::new('F', 0)),
                        (1, FaceJoin::new('B', 2)),
                        (2, FaceJoin::new('C', 3)),
                        (3, FaceJoin::new('D', 0)),
                    ]),
                ),
                (
                    'F',
                    HashMap::from([
                        (0, FaceJoin::new('A', 2)),
                        (1, FaceJoin::new('B', 1)),
                        (2, FaceJoin::new('E', 0)),
                        (3, FaceJoin::new('D', 1)),
                    ]),
                ),
            ]);
        }

        if !construct_cube {
            return HashMap::from([
                (
                    'A',
                    HashMap::from([
                        (0, FaceJoin::new('B', 0)),
                        (1, FaceJoin::new('C', 0)),
                        (2, FaceJoin::new('B', 0)),
                        (3, FaceJoin::new('E', 0)),
                    ]),
                ),
                (
                    'B',
                    HashMap::from([
                        (0, FaceJoin::new('A', 0)),
                        (1, FaceJoin::new('B', 0)),
                        (2, FaceJoin::new('A', 0)),
                        (3, FaceJoin::new('B', 0)),
                    ]),
                ),
                (
                    'C',
                    HashMap::from([
                        (0, FaceJoin::new('C', 0)),
                        (1, FaceJoin::new('E', 0)),
                        (2, FaceJoin::new('C', 0)),
                        (3, FaceJoin::new('A', 0)),
                    ]),
                ),
                (
                    'D',
                    HashMap::from([
                        (0, FaceJoin::new('E', 0)),
                        (1, FaceJoin::new('F', 0)),
                        (2, FaceJoin::new('E', 0)),
                        (3, FaceJoin::new('F', 0)),
                    ]),
                ),
                (
                    'E',
                    HashMap::from([
                        (0, FaceJoin::new('D', 0)),
                        (1, FaceJoin::new('A', 0)),
                        (2, FaceJoin::new('D', 0)),
                        (3, FaceJoin::new('C', 0)),
                    ]),
                ),
                (
                    'F',
                    HashMap::from([
                        (0, FaceJoin::new('F', 0)),
                        (1, FaceJoin::new('D', 0)),
                        (2, FaceJoin::new('F', 0)),
                        (3, FaceJoin::new('D', 0)),
                    ]),
                ),
            ]);
        }
        HashMap::from([
            (
                'A',
                HashMap::from([
                    (0, FaceJoin::new('B', 0)),
                    (1, FaceJoin::new('C', 0)),
                    (2, FaceJoin::new('D', 2)),
                    (3, FaceJoin::new('F', 3)),
                ]),
            ),
            (
                'B',
                HashMap::from([
                    (0, FaceJoin::new('E', 2)),
                    (1, FaceJoin::new('C', 3)),
                    (2, FaceJoin::new('A', 0)),
                    (3, FaceJoin::new('F', 0)),
                ]),
            ),
            (
                'C',
                HashMap::from([
                    (0, FaceJoin::new('B', 1)),
                    (1, FaceJoin::new('E', 0)),
                    (2, FaceJoin::new('D', 1)),
                    (3, FaceJoin::new('A', 0)),
                ]),
            ),
            (
                'D',
                HashMap::from([
                    (0, FaceJoin::new('E', 0)),
                    (1, FaceJoin::new('F', 0)),
                    (2, FaceJoin::new('A', 2)),
                    (3, FaceJoin::new('C', 3)),
                ]),
            ),
            (
                'E',
                HashMap::from([
                    (0, FaceJoin::new('B', 2)),
                    (1, FaceJoin::new('F', 3)),
                    (2, FaceJoin::new('D', 0)),
                    (3, FaceJoin::new('C', 0)),
                ]),
            ),
            (
                'F',
                HashMap::from([
                    (0, FaceJoin::new('E', 1)),
                    (1, FaceJoin::new('B', 0)),
                    (2, FaceJoin::new('A', 1)),
                    (3, FaceJoin::new('D', 0)),
                ]),
            ),
        ])
    }

    fn get_new_location(&self, current_position: &Position, instruction: &Instruction) -> Position {
        if instruction.turn.is_some() {
            return Position {
                location: current_position.location,
                direction: (current_position.direction + instruction.turn.unwrap() + 4) % 4,
                face: current_position.face,
            };
        } else {
            let directions: HashMap<isize, Point> = HashMap::from([
                (0, Point::new(1, 0)),
                (1, Point::new(0, 1)),
                (2, Point::new(-1, 0)),
                (3, Point::new(0, -1)),
            ]);
            let mut direction_vector = directions.get(&current_position.direction).unwrap();

            let mut new_position: Position = current_position.clone();

            for _ in 0..instruction.distance.unwrap() {
                let mut next_position = Position {
                    location: Point::add(&new_position.location, direction_vector),
                    direction: new_position.direction,
                    face: new_position.face,
                };

                if next_position.location.x < 1
                    || next_position.location.x > self.size
                    || next_position.location.y < 1
                    || next_position.location.y > self.size
                {
                    next_position = self.wrap_to_next_face(&next_position);

                    direction_vector = directions.get(&next_position.direction).unwrap();
                }

                if self
                    .faces
                    .get(&next_position.face)
                    .unwrap()
                    .contains(&next_position.location)
                {
                    break;
                } else {
                    new_position = next_position;
                }
            }

            return new_position;
        }
    }

    fn wrap_to_next_face(&self, next_position: &Position) -> Position {
        let mut wrapped_point = next_position.location;

        if wrapped_point.x < 1 {
            wrapped_point.x = self.size;
        }
        if wrapped_point.x > self.size {
            wrapped_point.x = 1;
        }
        if wrapped_point.y < 1 {
            wrapped_point.y = self.size;
        }
        if wrapped_point.y > self.size {
            wrapped_point.y = 1;
        }

        let target_join: &FaceJoin = self
            .face_joins
            .get(&next_position.face)
            .unwrap()
            .get(&next_position.direction)
            .unwrap();

        let new_direction = (next_position.direction - target_join.orientation + 4) % 4;

        let new_point: Point = match target_join.orientation {
            0 => wrapped_point,
            1 => Point::new(wrapped_point.y, self.size + 1 - wrapped_point.x),
            2 => Point::new(
                self.size + 1 - wrapped_point.x,
                self.size + 1 - wrapped_point.y,
            ),
            3 => Point::new(self.size + 1 - wrapped_point.y, wrapped_point.x),
            _ => panic!("Unexpected direction"),
        };

        Position {
            location: new_point,
            direction: new_direction,
            face: target_join.face_id,
        }
    }
}
