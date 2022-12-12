use crate::days::day::Day;

pub struct Day10 {}

impl Day for Day10 {
    fn solve_a(&self, file: &String) -> String {
        let mut time_step: isize = 0;
        let mut register_value: isize = 1;
        let mut result: isize = 0;

        for line in file.lines() {
            let commands = if line == "noop" {
                vec!["noop"]
            } else {
                vec!["noop", line]
            };

            for command in commands {
                time_step += 1;
                if (time_step + 20) % 40 == 0 && time_step <= 220 {
                    result += time_step * register_value;
                }

                if command.starts_with("addx") {
                    register_value += command
                        .split_at(5)
                        .1
                        .parse::<isize>()
                        .expect("Invalid input");
                }
            }
        }
        result.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut time_step: isize = 0;
        let mut register_value: isize = 1;

        for line in file.lines() {
            let commands = if line == "noop" {
                vec!["noop"]
            } else {
                vec!["noop", line]
            };

            for command in commands {
                if ((time_step % 40) - register_value).abs() <= 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
                if time_step % 40 == 39 {
                    print!("\n");
                }
                time_step += 1;
                if command.starts_with("addx") {
                    register_value += command
                        .split_at(5)
                        .1
                        .parse::<isize>()
                        .expect("Invalid input");
                }
            }
        }

        String::from("See output above")
    }
}
