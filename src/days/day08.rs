use std::collections::{HashSet, HashMap};

use crate::days::day::Day;

pub struct Day08 {}

impl Day for Day08 {
    fn solve_a(&self, file: &String) -> String {
        let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

        let lines: Vec<&str> = file.lines().collect();

        let width = lines[0].len();
        let height = lines.len();

        for x in 0..width {
            visible_trees.insert((x, 0));
            visible_trees.insert((x, height-1));
        }
        for y in 0..height {
            visible_trees.insert((0, y));
            visible_trees.insert((width-1, y));
        }

        for row_index in 1..height-1 {
            let row = lines[row_index].to_string();

            let mut left_index = 0;
            let mut left_height = string_get_nth(&row, left_index);
            let mut right_index = width - 1;
            let mut right_height = string_get_nth(&row, right_index);

            while left_index < right_index {
                if left_height < right_height {
                    left_index += 1;
                    let tree = string_get_nth(&row, left_index);

                    if tree > left_height {
                        visible_trees.insert((left_index, row_index));
                        left_height = tree;
                    }
                } else {
                    right_index -= 1;
                    let tree = string_get_nth(&row, right_index);

                    if tree > right_height {
                        visible_trees.insert((right_index, row_index));
                        right_height = tree;
                    }
                }
            }
        }
        for col_index in 1..width-1 {
            let col = lines.iter().map(|line| str_get_nth(&line, col_index)).collect();

            let mut top_index = 0;
            let mut top_height = string_get_nth(&col, top_index);
            let mut bottom_index = width - 1;
            let mut bottom_height = string_get_nth(&col, bottom_index);

            while top_index < bottom_index {
                if top_height < bottom_height {
                    top_index += 1;
                    let tree = string_get_nth(&col, top_index);

                    if tree > top_height {
                        visible_trees.insert((col_index, top_index));
                        top_height = tree;
                    }
                } else {
                    bottom_index -= 1;
                    let tree = string_get_nth(&col, bottom_index);

                    if tree > bottom_height {
                        visible_trees.insert((col_index, bottom_index));
                        bottom_height = tree;
                    }
                }
            }
        }

        visible_trees.len().to_string()
    }
    
    fn solve_b(&self, file: &String) -> String {
        let mut tree_visibilities: HashMap<(isize, isize), isize> = HashMap::new();

        let lines: Vec<&str> = file.lines().collect();

        let width: isize = lines[0].len().try_into().unwrap();
        let height: isize = lines.len().try_into().unwrap();

        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for x in 1..width-1 {
            for y in 1..height-1 {
                let tree_height = get_tree_height(&lines, x, y);
                let mut tree_vis = 1;
                for direction in &directions {
                    let mut x_pos = x;
                    let mut y_pos = y;
                    while x_pos > 0 && x_pos < width-1 && y_pos > 0 && y_pos < height-1 {
                        x_pos += direction.0;
                        y_pos += direction.1;

                        if get_tree_height(&lines, x_pos, y_pos) >= tree_height {
                            break;
                        }
                    }
                    let distance = direction.0 * (x_pos - x) + direction.1 * (y_pos - y);
                    tree_vis *= distance;
                }
                tree_visibilities.insert((x, y), tree_vis);
            }
        }

        tree_visibilities.values().max().unwrap().to_string()
    }
}

fn get_tree_height(lines: &Vec<&str>, x_pos: isize, y_pos: isize) -> char {
    lines.iter().nth(y_pos.try_into().unwrap()).unwrap().chars().nth(x_pos.try_into().unwrap()).unwrap()
}

fn str_get_nth(string: &str, index: usize) -> char {
    string.chars().nth(index).unwrap()
}

fn string_get_nth(string: &String, index: usize) -> char {
    string.chars().nth(index).unwrap()
}
