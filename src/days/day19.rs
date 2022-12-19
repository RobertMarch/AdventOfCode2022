use crate::days::day::Day;

use regex::{Captures, Regex};

pub struct Day19 {}

impl Day for Day19 {
    fn solve_a(&self, file: &String) -> String {
        let parsing_regex =
            Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
                .unwrap();

        let mut result: isize = 0;

        for line in file.lines() {
            let blueprint = Blueprint::parse(line, &parsing_regex);

            let initial_robots = Robots {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            };
            let initial_materials = Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };

            let max_geodes = get_max_geodes(
                24,
                &RobotType::None,
                &initial_materials,
                &initial_robots,
                &blueprint,
            );

            println!("Max geodes for {}: {max_geodes}", blueprint.id_number);
            result += blueprint.id_number * max_geodes;
        }

        result.to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let parsing_regex =
            Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
                .unwrap();

        let mut result: isize = 1;

        for line in file.lines() {
            let blueprint = Blueprint::parse(line, &parsing_regex);
            if blueprint.id_number > 3 {
                break;
            }

            let initial_robots = Robots {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            };
            let initial_materials = Materials {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };

            let max_geodes = get_max_geodes(
                32,
                &RobotType::None,
                &initial_materials,
                &initial_robots,
                &blueprint,
            );

            println!("Max geodes for {}: {max_geodes}", blueprint.id_number);
            result *= max_geodes;
        }

        result.to_string()
    }
}

enum RobotType {
    None,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn get_max_geodes(
    time_remaining: isize,
    last_action: &RobotType,
    current_materials: &Materials,
    current_robots: &Robots,
    blueprints: &Blueprint,
) -> isize {
    if time_remaining == 0 {
        return current_materials.geode;
    }

    get_options(
        time_remaining,
        last_action,
        current_materials,
        current_robots,
        blueprints,
    )
    .iter()
    .map(|option| {
        let mut new_materials = current_robots.harvest(current_materials);

        if matches!(option, RobotType::None) {
            return get_max_geodes(
                time_remaining - 1,
                option,
                &new_materials,
                current_robots,
                blueprints,
            );
        }

        new_materials = blueprints.pay_for_robot(option, &new_materials);

        let new_robots = blueprints.build_paid_for_robot(option, current_robots);

        get_max_geodes(
            time_remaining - 1,
            option,
            &new_materials,
            &new_robots,
            blueprints,
        )
    })
    .max()
    .unwrap()
}

fn get_options(
    time_remaining: isize,
    last_action: &RobotType,
    current_materials: &Materials,
    current_robots: &Robots,
    blueprints: &Blueprint,
) -> Vec<RobotType> {
    let mut options: Vec<RobotType> = vec![RobotType::None];

    if time_remaining > 1 {
        if include_ore_option(last_action, current_materials, current_robots, blueprints) {
            options.push(RobotType::Ore)
        }
        if include_clay_option(last_action, current_materials, current_robots, blueprints) {
            options.push(RobotType::Clay)
        }
        if include_obsidian_option(last_action, current_materials, current_robots, blueprints) {
            options.push(RobotType::Obsidian)
        }
        if current_materials.contains_at_least(&blueprints.geode_robot_cost) {
            options.push(RobotType::Geode)
        }
    }

    options.reverse();

    options
}

fn include_ore_option(
    last_action: &RobotType,
    current_materials: &Materials,
    current_robots: &Robots,
    blueprints: &Blueprint,
) -> bool {
    if !current_materials.contains_at_least(&blueprints.ore_robot_cost)
        || current_robots.ore_robots >= blueprints.get_max_ore_required()
    {
        return false;
    }

    if matches!(last_action, RobotType::None) {
        let materials_last_round = current_materials.ore - current_robots.ore_robots;
        return materials_last_round < blueprints.ore_robot_cost.ore;
    }

    true
}

fn include_clay_option(
    last_action: &RobotType,
    current_materials: &Materials,
    current_robots: &Robots,
    blueprints: &Blueprint,
) -> bool {
    if !current_materials.contains_at_least(&blueprints.clay_robot_cost)
        || current_robots.clay_robots >= blueprints.obsidian_robot_cost.clay
    {
        return false;
    }

    if matches!(last_action, RobotType::None) {
        let materials_last_round = current_materials.ore - current_robots.ore_robots;
        return materials_last_round < blueprints.clay_robot_cost.ore;
    }

    true
}

fn include_obsidian_option(
    last_action: &RobotType,
    current_materials: &Materials,
    current_robots: &Robots,
    blueprints: &Blueprint,
) -> bool {
    if !current_materials.contains_at_least(&blueprints.obsidian_robot_cost)
        || current_robots.obsidian_robots >= blueprints.geode_robot_cost.obsidian
    {
        return false;
    }

    if matches!(last_action, RobotType::None) {
        let ore_last_round = current_materials.ore - current_robots.ore_robots;
        let clay_last_round = current_materials.clay - current_robots.clay_robots;
        return ore_last_round < blueprints.obsidian_robot_cost.ore
            || clay_last_round < blueprints.obsidian_robot_cost.clay;
    }

    true
}

#[derive(Debug)]
struct Materials {
    ore: isize,
    clay: isize,
    obsidian: isize,
    geode: isize,
}

impl Materials {
    fn new(ore: isize, clay: isize, obsidian: isize, geode: isize) -> Materials {
        Materials {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn clone(&self) -> Self {
        Materials {
            ore: self.ore,
            clay: self.clay,
            obsidian: self.obsidian,
            geode: self.geode,
        }
    }

    fn contains_at_least(&self, other: &Materials) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn subtract(&self, other: &Materials) -> Materials {
        Materials {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

#[derive(Debug)]
struct Robots {
    ore_robots: isize,
    clay_robots: isize,
    obsidian_robots: isize,
    geode_robots: isize,
}

impl Robots {
    fn harvest(&self, current_materials: &Materials) -> Materials {
        Materials {
            ore: current_materials.ore + self.ore_robots,
            clay: current_materials.clay + self.clay_robots,
            obsidian: current_materials.obsidian + self.obsidian_robots,
            geode: current_materials.geode + self.geode_robots,
        }
    }

    fn clone(&self) -> Self {
        Robots {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id_number: isize,
    ore_robot_cost: Materials,
    clay_robot_cost: Materials,
    obsidian_robot_cost: Materials,
    geode_robot_cost: Materials,
}

impl Blueprint {
    fn parse(input: &str, parsing_regex: &Regex) -> Blueprint {
        let groups = parsing_regex.captures(input).unwrap();

        let id_number = parse_regex_match(&groups, 1);
        let ore_robot_cost = Materials::new(parse_regex_match(&groups, 2), 0, 0, 0);
        let clay_robot_cost = Materials::new(parse_regex_match(&groups, 3), 0, 0, 0);
        let obsidian_robot_cost = Materials::new(
            parse_regex_match(&groups, 4),
            parse_regex_match(&groups, 5),
            0,
            0,
        );
        let geode_robot_cost = Materials::new(
            parse_regex_match(&groups, 6),
            0,
            parse_regex_match(&groups, 7),
            0,
        );

        Blueprint {
            id_number,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }

    fn get_max_ore_required(&self) -> isize {
        vec![
            self.ore_robot_cost.ore,
            self.clay_robot_cost.ore,
            self.obsidian_robot_cost.ore,
            self.geode_robot_cost.ore,
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
    }

    fn pay_for_robot(&self, robot_type: &RobotType, current_materials: &Materials) -> Materials {
        match robot_type {
            RobotType::Ore => current_materials.subtract(&self.ore_robot_cost),
            RobotType::Clay => current_materials.subtract(&self.clay_robot_cost),
            RobotType::Obsidian => current_materials.subtract(&self.obsidian_robot_cost),
            RobotType::Geode => current_materials.subtract(&self.geode_robot_cost),
            RobotType::None => current_materials.clone(),
        }
    }

    fn build_paid_for_robot(&self, robot_type: &RobotType, current_robots: &Robots) -> Robots {
        let mut new_robots = current_robots.clone();
        match robot_type {
            RobotType::Ore => {
                new_robots.ore_robots += 1;
                new_robots
            }
            RobotType::Clay => {
                new_robots.clay_robots += 1;
                new_robots
            }
            RobotType::Obsidian => {
                new_robots.obsidian_robots += 1;
                new_robots
            }
            RobotType::Geode => {
                new_robots.geode_robots += 1;
                new_robots
            }
            RobotType::None => new_robots,
        }
    }
}

fn parse_regex_match(regex_capture: &Captures, group: usize) -> isize {
    regex_capture
        .get(group)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .expect("Unexpected value when parsing regex")
}
