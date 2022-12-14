use std::collections::HashSet;

use crate::days::day::Day;

pub struct Day14 {}

impl Day for Day14 {
    fn solve_a(&self, file: &String) -> String {
        let mut filled_points: HashSet<Point> = parse_input(file);
        let rock_length: usize = filled_points.len();

        let max_rock_depth: isize = filled_points.iter().map(|point| point.y).max().unwrap();

        loop {
            let sand_location = drop_sand(&filled_points, max_rock_depth);

            if sand_location.y > max_rock_depth {
                break;
            }

            filled_points.insert(sand_location);
        }

        (filled_points.len() - rock_length).to_string()
    }
    
    fn solve_b(&self, file: &String) -> String {
        let mut filled_points: HashSet<Point> = parse_input(file);
        let rock_length: usize = filled_points.len();

        let max_rock_depth: isize = filled_points.iter().map(|point| point.y).max().unwrap();

        loop {
            let sand_location = drop_sand(&filled_points, max_rock_depth);

            if sand_location.y == 0 {
                filled_points.insert(sand_location);
                break;
            }

            filled_points.insert(sand_location);
        }

        (filled_points.len() - rock_length).to_string()
    }
}

fn drop_sand(filled_points: &HashSet<Point>, max_rock_depth: isize) -> Point {
    let sand_moves: Vec<Point> = vec![
        Point::new(0, 1),
        Point::new(-1, 1),
        Point::new(1, 1),
    ];

    let mut sand_location = Point::new(500, 0);
    let mut can_move = true;

    while can_move {
        can_move = false;

        'sand_move: for possible_move in &sand_moves {
            let new_position = add_points(&sand_location, &possible_move);

            if new_position.y < max_rock_depth + 2 && !filled_points.contains(&new_position) {
                can_move = true;
                sand_location = new_position;
                break 'sand_move;
            }
        }
    }

    sand_location
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

    fn from_string(input_point: &str) -> Point {
        let (x, y) = input_point.split_once(",").unwrap();

        Point::new(x.parse().expect(""), y.parse().expect(""))
    }
}

fn add_points(a: &Point, b: &Point) -> Point {
    Point { 
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

fn parse_input(file: &String) -> HashSet<Point> {
    let mut rock_points: HashSet<Point> = HashSet::new();

    for line in file.lines() {
        let corners: Vec<Point> = line.split(" -> ").map(|corner_string| Point::from_string(corner_string)).collect();

        for i in 0..corners.len()-1 {
            let start = corners.get(i).unwrap();
            let end = corners.get(i + 1).unwrap();

            for x in start.x.min(end.x)..start.x.max(end.x)+1 {
                for y in start.y.min(end.y)..start.y.max(end.y)+1 {
                    rock_points.insert(Point::new(x, y));
                }
            }
        }
    }

    rock_points
}
