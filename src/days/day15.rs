use std::collections::HashSet;

use regex::{Captures, Regex};

use crate::days::day::Day;

pub struct Day15 {}

impl Day for Day15 {
    fn solve_a(&self, file: &String) -> String {
        let (sensors, beacons): (Vec<Sensor>, HashSet<Point>) = parse_sensors(file);

        let target_y: isize = 2000000;

        let ranges = get_merged_ranges_for_line(&sensors, target_y);

        let total_size: isize = ranges.iter().map(|range| range.end - range.start + 1).sum();
        let beacons_on_target_line: usize = beacons
            .iter()
            .filter(|beacon| beacon.y == target_y)
            .collect::<Vec<&Point>>()
            .len();

        (total_size as usize - beacons_on_target_line).to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let (sensors, _beacons): (Vec<Sensor>, HashSet<Point>) = parse_sensors(file);

        for target_y in 0..40000001 {
            let ranges = get_merged_ranges_for_line(&sensors, target_y);

            let filtered_ranges: Vec<&Range> = ranges
                .iter()
                .filter(|range| range.start <= 4000000 && range.end >= 0)
                .collect();
            let first_range: &Range = filtered_ranges.get(0).unwrap();

            if filtered_ranges.len() > 1 {
                let x = first_range.start.max(filtered_ranges.get(1).unwrap().start) - 1;
                return (x * 4000000 + target_y).to_string();
            }
            if first_range.start > 0 {
                return 0.to_string();
            }
            if first_range.end < 4000000 {
                return 4000000.to_string();
            }
        }

        String::from("No solution found")
    }
}

fn get_merged_ranges_for_line(sensors: &Vec<Sensor>, target_y: isize) -> Vec<Range> {
    let mut target_line_ranges: Vec<Range> = vec![];

    for sensor in sensors {
        let sensor_target_row_overlap = sensor.range - (sensor.location.y - target_y).abs();

        if sensor_target_row_overlap > 0 {
            target_line_ranges.push(Range {
                start: sensor.location.x - sensor_target_row_overlap,
                end: sensor.location.x + sensor_target_row_overlap,
            });
        }
    }

    merge_all_ranges(target_line_ranges)
}

fn parse_sensors(file: &String) -> (Vec<Sensor>, HashSet<Point>) {
    let parsing_regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    let mut sensors: Vec<Sensor> = vec![];
    let mut beacons: HashSet<Point> = HashSet::new();

    for line in file.lines() {
        let groups = parsing_regex.captures(line).unwrap();

        let sensor = Point::new(parse_regex_match(&groups, 1), parse_regex_match(&groups, 2));
        let beacon = Point::new(parse_regex_match(&groups, 3), parse_regex_match(&groups, 4));

        let manhatten_dist = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        sensors.push(Sensor::new(sensor, manhatten_dist));
        beacons.insert(beacon);
    }

    (sensors, beacons)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    range: isize,
}

impl Sensor {
    fn new(location: Point, range: isize) -> Sensor {
        Sensor { location, range }
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn merge(&mut self, other: Range) -> bool {
        if self.start <= other.end && self.end >= other.start {
            self.start = self.start.min(other.start);
            self.end = self.end.max(other.end);
            return true;
        }
        false
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

fn merge_all_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut new_ranges: Vec<Range> = vec![];
    let mut has_combination: bool = false;

    for range in ranges {
        let mut is_distinct = true;
        for new_range in &mut new_ranges {
            if new_range.merge(range) {
                is_distinct = false;
                has_combination = true;
            }
        }

        if is_distinct {
            new_ranges.push(range);
        }
    }

    if has_combination {
        merge_all_ranges(new_ranges)
    } else {
        new_ranges
    }
}
