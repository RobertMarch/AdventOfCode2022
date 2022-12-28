use crate::days::day::Day;

pub struct Day25 {}

impl Day for Day25 {
    fn solve_a(&self, file: &String) -> String {
        let mut result = 0;
    
        for line in file.lines() {
            let mut chars: Vec<char> = line.chars().collect();
            chars.reverse();
            
            let mut line_result = 0;
            for (i, c) in chars.iter().enumerate() {
                let char_value: isize = match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("Unexpected value")
                };
                
                line_result += 5_isize.pow(i as u32) * char_value;
            }
            
            result += line_result;
        }
        
        let mut result_snafu: Vec<char> = vec![];
        
        while result != 0 {
            let res_part = result % 5;
            result = result / 5;
            
            match res_part {
                0 => result_snafu.push('0'),
                1 => result_snafu.push('1'),
                2 => result_snafu.push('2'),
                3 => {
                    result_snafu.push('=');
                    result += 1;
                },
                4 => {
                    result_snafu.push('-');
                    result += 1;
                },
                _ => panic!("Unexpected value"),
            };
        }
        
        result_snafu.reverse();

        result_snafu.iter().collect::<String>()
    }

    fn solve_b(&self, _file: &String) -> String {
        String::from("No part two")
    }
}
