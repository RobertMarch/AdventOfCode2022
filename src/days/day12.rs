use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day12 {}

impl Day for Day12 {
    fn solve_a(&self, file: &String) -> String {
        let (grid, start, end) = parse_grid(file);

        solve(grid, start, 1, end, -1)
    }

    fn solve_b(&self, file: &String) -> String {
        let (grid, _start, end) = parse_grid(file);
        let target_value = 'a' as isize;

        solve(grid, end, -1, Point::new(-1, -1), target_value)
    }
}

fn solve(grid: Grid, start_point: Point, search_direction: isize, target_point: Point, target_value: isize) -> String {
    let mut known_points: DistanceToPoints = HashMap::new();
    let mut neighbour_points: DistanceToPoints = HashMap::new();
    neighbour_points.insert(start_point, 0);
    add_to_known_points(&start_point, search_direction, &mut known_points, &mut neighbour_points, &grid);
    
    while neighbour_points.len() > 0 {
        let next_point = neighbour_points.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap().clone();
        
        add_to_known_points(&next_point, search_direction, &mut known_points, &mut neighbour_points, &grid);

        if target_point.eq(&next_point) || target_value.eq(grid.get(&next_point).unwrap()) {
            return known_points.get(&next_point).unwrap().to_string();
        }
    }

    String::from("Path not found")
}

fn add_to_known_points(point: &Point, search_direction: isize, known_points: &mut DistanceToPoints, neighbour_points: &mut DistanceToPoints, grid: &Grid) {
    if known_points.contains_key(&point) {
        panic!("Unexpected point, have already visited: {:?}", point);
    }

    let distance_to_point = neighbour_points.get(&point).unwrap().clone();

    known_points.insert(point.clone(), distance_to_point);

    neighbour_points.remove(&point);

    let neighbours = get_neighbours(&point, grid);

    for neighbour in neighbours {
        if known_points.contains_key(&neighbour) {
            continue;
        }
        if neighbour_points.contains_key(&neighbour) && neighbour_points.get(&neighbour).unwrap() < &distance_to_point {
            continue;
        }
        if (grid.get(&neighbour).unwrap() - grid.get(point).unwrap()) * search_direction <= 1 {
            neighbour_points.insert(neighbour, distance_to_point + 1);
        }
    }
}

fn get_neighbours(point: &Point, grid: &Grid) -> Vec<Point> {
    let directions: Vec<Point> = vec![
        Point::new(-1, 0),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(0, 1),
    ];

    directions.iter()
        .map(|direction| add_points(point, &direction))
        .filter(|new_point| grid.contains_key(new_point))
        .collect()
}

fn add_points(a: &Point, b: &Point) -> Point {
    Point { 
        x: a.x + b.x,
        y: a.y + b.y,
    }
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

    fn new_from_usize(x: usize, y: usize) -> Point {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

type Grid = HashMap<Point, isize>;
type DistanceToPoints = HashMap<Point, usize>;

fn parse_grid(input: &str) -> (Grid, Point, Point) {
    let mut grid: Grid = HashMap::new();
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        for (x, mut char) in line.char_indices() {
            if char == 'S' {
                start = Some(Point::new_from_usize(x, y));
                char = 'a';
            }
            if char == 'E' {
                end = Some(Point::new_from_usize(x, y));
                char = 'z';
            }

            grid.insert(Point::new_from_usize(x, y), char as isize);
        }
    }

    if start.is_none() || end.is_none() {
        panic!("Did not find start");
    }

    (grid, start.unwrap(), end.unwrap())
}

