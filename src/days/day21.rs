use std::collections::HashMap;

use crate::days::day::Day;

pub struct Day21 {}

impl Day for Day21 {
    fn solve_a(&self, file: &String) -> String {
        solve(file, "root", false)
    }

    fn solve_b(&self, file: &String) -> String {
        solve(file, "humn", true)
    }
}

fn solve(file: &String, target_monkey: &str, is_part_b: bool) -> String {
    let mut monkeys: Monkeys = parse_monkeys(file, is_part_b);

    loop {
        for monkey_id in monkeys.clone().keys() {
            let old_monkey = monkeys.get(monkey_id).unwrap().clone();
            old_monkey.evaluate(&mut monkeys);
        }
        if monkeys.get(target_monkey).unwrap().known_value.is_some() {
            return monkeys.get(target_monkey).unwrap().known_value.unwrap().to_string();
        }
    }

}

fn parse_monkeys(file: &String, is_part_b: bool) -> Monkeys {
    let mut monkeys: Monkeys = HashMap::new();
    for line in file.lines() {
        let monkey_id = line[..4].to_string();
        if is_part_b && monkey_id.eq("humn") {
            monkeys.insert(
                monkey_id.to_string(),
                Monkey::new_operation_monkey(monkey_id, "humn", "humn", '#'),
            );
        } else if line.len() == 17 {
            let lhs_id = &line[6..10];
            let rhs_id = &line[13..];
            let operator = if is_part_b && monkey_id.eq("root") {
                '='
            } else {
                line.chars().nth(11).unwrap()
            };

            monkeys.insert(
                monkey_id.to_string(),
                Monkey::new_operation_monkey(monkey_id, lhs_id, rhs_id, operator),
            );
        } else {
            let monkey_value: isize = line[6..]
                .parse()
                .expect("Could not parse monkey value to number");

            monkeys.insert(
                monkey_id.to_string(),
                Monkey::new_value_monkey(monkey_id, monkey_value)
            );
        }
    }
    
    monkeys
}

type Monkeys = HashMap<String, Monkey>;

#[derive(Debug, Clone)]
struct Monkey {
    monkey_id: String,
    operation: Option<Operation>,
    known_value: Option<isize>,
}

#[derive(Debug, Clone)]
struct Operation {
    lhs_id: String,
    rhs_id: String,
    operator: char,
}

impl Monkey {
    fn new_operation_monkey(
        monkey_id: String,
        lhs_id: &str,
        rhs_id: &str,
        operator: char,
    ) -> Monkey {
        Monkey {
            monkey_id: monkey_id.to_string(),
            operation: Some(Operation {
                lhs_id: lhs_id.to_string(),
                rhs_id: rhs_id.to_string(),
                operator: operator,
            }),
            known_value: None,
        }
    }

    fn new_value_monkey(
        monkey_id: String,
        value: isize,
    ) -> Monkey {
        Monkey {
            monkey_id: monkey_id.to_string(),
            operation: None,
            known_value: Some(value),
        }
    }

    fn can_evaluate(&self, monkeys: &mut Monkeys) -> bool {
        if self.operation.is_none() {
            return false;
        }
        let operation = self.operation.clone().unwrap();
        let is_lhs_known = monkeys
            .get(&operation.lhs_id)
            .unwrap()
            .known_value
            .is_some();
        let is_rhs_known = monkeys
            .get(&operation.rhs_id)
            .unwrap()
            .known_value
            .is_some();
        let is_value_known = self.known_value.is_some();

        if is_lhs_known && is_rhs_known && is_value_known {
            return false;
        }

        if operation.operator.eq(&'=') {
            return is_lhs_known || is_rhs_known;
        }

        vec![is_lhs_known, is_rhs_known, is_value_known]
            .iter()
            .filter(|v| **v)
            .collect::<Vec<&bool>>()
            .len()
            == 2
    }

    fn evaluate(&self, monkeys: &mut Monkeys) {
        if !self.can_evaluate(monkeys) {
            return;
        }
        let operation = self.operation.clone().unwrap();
        let lhs_id = operation.lhs_id;
        let rhs_id = operation.rhs_id;

        if self.known_value.is_none() {
            if operation.operator.eq(&'=') {
                if monkeys.get(&lhs_id).unwrap().known_value.is_some() {
                    let lhs_value = monkeys.get(&lhs_id).unwrap().known_value.unwrap();
                    monkeys.get_mut(&self.monkey_id).unwrap().known_value = Some(lhs_value);
                    monkeys.get_mut(&rhs_id).unwrap().known_value = Some(lhs_value);
                } else {
                    let rhs_value = monkeys.get(&rhs_id).unwrap().known_value.unwrap();
                    monkeys.get_mut(&self.monkey_id).unwrap().known_value = Some(rhs_value);
                    monkeys.get_mut(&lhs_id).unwrap().known_value = Some(rhs_value);
                }
            } else {
                let lhs_value = monkeys.get(&lhs_id).unwrap().known_value.unwrap();
                let rhs_value = monkeys.get(&rhs_id).unwrap().known_value.unwrap();

                monkeys.get_mut(&self.monkey_id).unwrap().known_value =
                    Some(match operation.operator {
                        '+' => lhs_value + rhs_value,
                        '-' => lhs_value - rhs_value,
                        '*' => lhs_value * rhs_value,
                        '/' => lhs_value / rhs_value,
                        '=' => -1,
                        _ => panic!("Unexpected operator"),
                    });
            }
        } else {
            let self_value = self.known_value.unwrap();
            if monkeys.get(&lhs_id).unwrap().known_value.is_some() {
                let lhs_value = monkeys.get(&lhs_id).unwrap().known_value.unwrap();

                monkeys.get_mut(&rhs_id).unwrap().known_value = Some(match operation.operator {
                    '+' => self_value - lhs_value,
                    '-' => lhs_value - self_value,
                    '*' => self_value / lhs_value,
                    '/' => lhs_value / self_value,
                    '=' => -1,
                    _ => panic!("Unexpected operator"),
                });
            } else {
                let rhs_value = monkeys.get(&rhs_id).unwrap().known_value.unwrap();

                monkeys.get_mut(&lhs_id).unwrap().known_value = Some(match operation.operator {
                    '+' => self_value - rhs_value,
                    '-' => self_value + rhs_value,
                    '*' => self_value / rhs_value,
                    '/' => self_value * rhs_value,
                    '=' => -1,
                    _ => panic!("Unexpected operator"),
                });
            }
        }
    }
}
