use crate::days::day::Day;

pub struct Day20 {}

impl Day for Day20 {
    fn solve_a(&self, file: &String) -> String {
        solve(file, 1, 1)
    }

    fn solve_b(&self, file: &String) -> String {
        solve(file, 10, 811589153)
    }
}

fn solve(file: &String, mix_count: usize, decryption_key: isize) -> String {
    let mut values: Vec<Value> = vec![];

    for (i, line) in file.lines().enumerate() {
        values.push((
            line.parse::<isize>().expect("") * decryption_key,
            i as isize,
        ));
    }

    let total_values = values.len();

    for _ in 0..mix_count {
        for i in 0..total_values {
            let start_index = find_index_by_order(&values, i as isize);
            let element = values[start_index];

            values.remove(start_index);

            let mut new_index = (start_index as isize + element.0) % (values.len() as isize);
            if new_index < 0 {
                new_index += values.len() as isize;
            }

            values.insert(new_index as usize, element);
        }
    }

    let zero_index: usize = find_index_by_value(&values, 0);

    let mut result: isize = 0;

    result += values[(zero_index + 1000) % values.len()].0;
    result += values[(zero_index + 2000) % values.len()].0;
    result += values[(zero_index + 3000) % values.len()].0;

    result.to_string()
}

type Value = (isize, isize);

fn find_index_by_value(values: &Vec<Value>, target_value: isize) -> usize {
    for (i, val) in values.iter().enumerate() {
        if val.0 == target_value {
            return i;
        }
    }

    panic!("Entry not found with target value")
}

fn find_index_by_order(values: &Vec<Value>, initial_order: isize) -> usize {
    for (i, val) in values.iter().enumerate() {
        if val.1 == initial_order {
            return i;
        }
    }

    panic!("Entry not found with initial position")
}
