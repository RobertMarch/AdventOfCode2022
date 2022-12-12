use crate::days::day::Day;

pub struct Day11 {}

impl Day for Day11 {
    fn solve_a(&self, file: &String) -> String {
        solve(file, 20, 3)
    }

    fn solve_b(&self, file: &String) -> String {
        solve(file, 10000, 1)
    }
}

fn solve(file: &String, total_rounds: usize, stress_divisor: usize) -> String {
    let (monkeys, mut inventory) = Monkey::parse_all(file, stress_divisor);
    let mut counts: InspectionCount = [0; 8].to_vec();

    let monkey_test_lcm: usize = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _i in 0..total_rounds {
        monkeys.iter().for_each(|monkey| {
            monkey.inspect_and_move(&mut inventory, &mut counts, monkey_test_lcm)
        });
    }

    println!("{:?}", counts);

    counts.sort();
    counts.reverse();

    (counts[0] * counts[1]).to_string()
}

#[derive(Debug)]
struct Monkey {
    number: usize,
    operation: String,
    operation_rhs: usize,
    test_divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
    stress_divisor: usize,
}

type Monkeys = Vec<Monkey>;

#[derive(Debug)]
struct ItemPosition {
    monkey_number: usize,
    value: usize,
}

type Inventory = Vec<ItemPosition>;
type InspectionCount = Vec<usize>;

impl Monkey {
    fn inspect_and_move(
        &self,
        inventory: &mut Inventory,
        counts: &mut InspectionCount,
        monkey_test_lcm: usize,
    ) {
        for item in inventory {
            if item.monkey_number == self.number {
                item.value = (self.operate(item) / self.stress_divisor) % monkey_test_lcm;
                item.monkey_number = if item.value % self.test_divisor == 0 {
                    self.true_monkey
                } else {
                    self.false_monkey
                };
                counts[self.number] += 1;
            }
        }
    }

    fn operate(&self, item: &ItemPosition) -> usize {
        if self.operation.eq("sq") {
            return item.value.pow(2);
        } else if self.operation.eq("+") {
            return item.value + self.operation_rhs;
        } else {
            return item.value * self.operation_rhs;
        }
    }

    fn parse_all(input: &str, stress_divisor: usize) -> (Monkeys, Inventory) {
        let mut monkeys: Monkeys = vec![];
        let mut inventory: Inventory = vec![];
        input.split("\n\n").for_each(|monkey| {
            let (monkey, items) = Monkey::parse(monkey, stress_divisor);
            monkeys.push(monkey);
            for item in items {
                inventory.push(item);
            }
        });
        (monkeys, inventory)
    }

    fn parse(input: &str, stress_divisor: usize) -> (Monkey, Inventory) {
        let lines: Vec<&str> = input.lines().collect();

        let operation: String = if lines[2][23..26].eq("* o") {
            "sq".to_string()
        } else {
            lines[2][23..24].to_string()
        };
        let operation_rhs: usize = lines[2][25..].parse().unwrap_or(0);

        let monkey = Monkey {
            number: lines[0].chars().nth_back(1).unwrap().to_digit(10).unwrap() as usize,
            operation,
            operation_rhs,
            test_divisor: lines[3]
                .split_once("divisible by ")
                .unwrap()
                .1
                .parse()
                .expect(""),
            true_monkey: lines[4].split_once("monkey ").unwrap().1.parse().expect(""),
            false_monkey: lines[5].split_once("monkey ").unwrap().1.parse().expect(""),
            stress_divisor,
        };

        let items: Inventory = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|val| ItemPosition {
                value: val.parse::<usize>().expect(""),
                monkey_number: monkey.number,
            })
            .collect();

        (monkey, items)
    }
}
