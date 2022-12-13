use crate::days::day::Day;

use serde_json::{self, json};

pub struct Day13 {}

impl Day for Day13 {
    fn solve_a(&self, file: &String) -> String {
        let mut result: usize = 0;

        for (pair_index, pair) in file.split("\n\n").enumerate() {
            let (first, second) = pair.split_once("\n").unwrap();

            let first_data: serde_json::Value = serde_json::from_str(first).unwrap();
            let second_data: serde_json::Value = serde_json::from_str(second).unwrap();

            let pair_result = are_pair_sorted(&first_data, &second_data);
            if pair_result.is_none() || pair_result.unwrap() {
                result += pair_index + 1;
            }
        }

        result.to_string()
    }
    
    fn solve_b(&self, _file: &String) -> String {
        String::from("Not yet implemented")
    }
}

fn are_pair_sorted(first: &serde_json::Value, second: &serde_json::Value) -> Option<bool> {
    if first.is_number() && second.is_number() {
        if first.as_u64() == second.as_u64() {
            return None;
        }
        return Some(first.as_u64() < second.as_u64());
    } else if first.is_number() && second.is_array() {
        return are_pair_sorted(&json!([first.as_u64()]), second);
    } else if first.is_array() && second.is_number() {
        return are_pair_sorted(first, &json!([second.as_u64()]));
    } else {
        let first_array = first.as_array().unwrap();
        let second_array = second.as_array().unwrap();
        
        for i in 0..first_array.len().min(second_array.len()) {
            let comp_result = are_pair_sorted(first_array.get(i).unwrap(), second_array.get(i).unwrap());
            
            if comp_result.is_some() {
                return comp_result;
            }
        }
        let len_diff = first_array.len().cmp(&second_array.len());
        
        return if len_diff.is_eq() { None } else { Some(len_diff.is_lt()) }
    }
}
