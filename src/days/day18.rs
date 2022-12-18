use std::collections::HashSet;

use crate::days::day::Day;

pub struct Day18 {}

impl Day for Day18 {
    fn solve_a(&self, file: &String) -> String {
        let rock_points = parse_input(file);

        count_rock_faces(&rock_points).to_string()
    }
    
    fn solve_b(&self, file: &String) -> String {
        let rock_points = parse_input(file);
        let rock_dimensions = RockDimensions::from_point_set(&rock_points);

        let air_points: HashSet<Point> = get_all_air(rock_dimensions.get_start_air_point(), &rock_points, &rock_dimensions);

        let all_rock: HashSet<Point> = &rock_dimensions.get_all_points() - &air_points;

        count_rock_faces(&all_rock).to_string()
    }
}

fn parse_input(input: &String) -> HashSet<Point> {
    let mut rock_points: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let parts: Vec<isize> = line.split(",").map(|coord| coord.parse::<isize>().expect("Unexpected value")).collect();
        rock_points.insert((parts[0], parts[1], parts[2]));
    }

    rock_points
}

fn count_rock_faces(rock_points: &HashSet<Point>) -> usize {
    let mut result = 0;

    for point in rock_points.iter() {
        for neighbour in get_neighbours(&point) {
            if !rock_points.contains(&neighbour) {
                result += 1;
            }
        }
    }

    result
}

fn get_all_air(start: Point, rock_points: &HashSet<Point>, rock_dimensions: &RockDimensions) -> HashSet<Point> {
    let mut air_points: HashSet<Point> = HashSet::new();
    let mut neighbours: HashSet<Point> = HashSet::new();
    neighbours.insert(start);

    while neighbours.len() > 0 {
        let next_point = neighbours.iter().next().unwrap().clone();

        for neighbour in get_neighbours(&next_point) {
            if rock_points.contains(&neighbour) || air_points.contains(&neighbour) || !rock_dimensions.is_in_range(&neighbour) {
                continue;
            }

            air_points.insert(neighbour);
            neighbours.insert(neighbour);
        }

        neighbours.remove(&next_point);
    }

    air_points
}

fn get_neighbours(point: &Point) -> Vec<Point> {
    vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ].iter()
    .map(|p| Point::add(&point, &p))
    .collect()
}

struct RockDimensions {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl RockDimensions {
    fn from_point_set(rock_points: &HashSet<Point>) -> Self {
        RockDimensions {
            min_x: rock_points.iter().map(|p| p.0).min().unwrap(),
            max_x: rock_points.iter().map(|p| p.0).max().unwrap(),
            min_y: rock_points.iter().map(|p| p.1).min().unwrap(),
            max_y: rock_points.iter().map(|p| p.1).max().unwrap(),
            min_z: rock_points.iter().map(|p| p.2).min().unwrap(),
            max_z: rock_points.iter().map(|p| p.2).max().unwrap(),
        }
    }

    fn is_in_range(&self, point: &Point) -> bool {
        self.min_x - 1 <= point.0 && point.0 <= self.max_x + 1
        && self.min_y - 1 <= point.1 && point.1 <= self.max_y + 1
        && self.min_z - 1 <= point.2 && point.2 <= self.max_z + 1
    }

    fn get_start_air_point(&self) -> Point {
        (self.min_x - 1, self.min_y - 1, self.min_z - 1)
    }

    fn get_all_points(&self) -> HashSet<Point> {
        let mut all_points: HashSet<Point> = HashSet::new();

        for x in self.min_x-1..self.max_x+1 {
            for y in self.min_y-1..self.max_y+1 {
                for z in self.min_z-1..self.max_z+1 {
                    all_points.insert((x, y, z));
                }
            }
        }

        all_points
    }
}

type Point = (isize, isize, isize);

trait PointMethods {
    fn add(a: &Point, b: &Point) -> Point;
}

impl PointMethods for Point {
    fn add(a: &Point, b: &Point) -> Point {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2)
    }
}
