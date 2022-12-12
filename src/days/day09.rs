use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day09 {}

impl Day for Day09 {
    fn solve_a(&self, file: &String) -> String {
        simulate_rope(file, 2)
    }

    fn solve_b(&self, file: &String) -> String {
        simulate_rope(file, 10)
    }
}

fn simulate_rope(file: &String, rope_length: usize) -> String {
    let directions: HashMap<char, (isize, isize)> =
        HashMap::from([('R', (1, 0)), ('L', (-1, 0)), ('U', (0, 1)), ('D', (0, -1))]);

    let mut visited_points: HashSet<(isize, isize)> = HashSet::from([(0, 0)]);

    let mut rope_segments: Vec<(isize, isize)> = vec![(0, 0); rope_length];

    for line in file.lines() {
        let (direction, distance) = line.split_once(" ").unwrap();
        let distance: isize = distance.parse().expect("Unexpected distance");
        let direction: char = direction.chars().nth(0).unwrap();

        let dir_vector = directions.get(&direction).unwrap();

        for _ in 0..distance {
            rope_segments.get_mut(0).unwrap().0 += dir_vector.0;
            rope_segments.get_mut(0).unwrap().1 += dir_vector.1;

            for segment in 1..rope_length {
                let rope_x_length = rope_segments.get(segment - 1).unwrap().0
                    - rope_segments.get(segment).unwrap().0;
                let rope_y_length = rope_segments.get(segment - 1).unwrap().1
                    - rope_segments.get(segment).unwrap().1;

                if rope_x_length.abs() == 2 || rope_y_length.abs() == 2 {
                    rope_segments.get_mut(segment).unwrap().0 += rope_x_length.signum();
                    rope_segments.get_mut(segment).unwrap().1 += rope_y_length.signum();

                    if segment == rope_length - 1 {
                        visited_points.insert(rope_segments.get_mut(segment).unwrap().clone());
                    }
                }
            }
        }
    }

    visited_points.len().to_string()
}
