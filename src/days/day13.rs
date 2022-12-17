use std::cmp::Ordering;

use crate::days::day::Day;

use serde_json::{self, json, Value};

pub struct Day13 {}

impl Day for Day13 {
    fn solve_a(&self, file: &String) -> String {
        let mut result: usize = 0;

        for (pair_index, pair) in file.split("\n\n").enumerate() {
            let (first, second) = pair.split_once("\n").unwrap();

            let first_data: serde_json::Value = serde_json::from_str(first).unwrap();
            let second_data: serde_json::Value = serde_json::from_str(second).unwrap();

            if are_pair_sorted(&first_data, &second_data).is_le() {
                result += pair_index + 1;
            }
        }

        result.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut packets: Vec<serde_json::Value> = file
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .map(|v| serde_json::from_str(v).unwrap())
            .collect();
        let divider_2: Value = "[[2]]".parse().expect("Failed to create divider packet");
        let divider_6: Value = "[[6]]".parse().expect("Failed to create divider packet");
        packets.push(divider_2.clone());
        packets.push(divider_6.clone());

        packets.sort_by(are_pair_sorted);

        let divider_2_position = packets
            .iter()
            .position(|packet| packet.to_string() == divider_2.to_string())
            .unwrap()
            + 1;
        let divider_6_position = packets
            .iter()
            .position(|packet| packet.to_string() == divider_6.to_string())
            .unwrap()
            + 1;

        (divider_2_position * divider_6_position).to_string()
    }
}

fn are_pair_sorted(first: &serde_json::Value, second: &serde_json::Value) -> Ordering {
    if first.is_number() && second.is_number() {
        return first.as_u64().cmp(&second.as_u64());
    } else if first.is_number() && second.is_array() {
        return are_pair_sorted(&json!([first.as_u64()]), second);
    } else if first.is_array() && second.is_number() {
        return are_pair_sorted(first, &json!([second.as_u64()]));
    } else {
        let first_array = first.as_array().unwrap();
        let second_array = second.as_array().unwrap();

        for i in 0..first_array.len().min(second_array.len()) {
            let comp_result =
                are_pair_sorted(first_array.get(i).unwrap(), second_array.get(i).unwrap());

            if !comp_result.is_eq() {
                return comp_result;
            }
        }
        return first_array.len().cmp(&second_array.len());
    }
}
