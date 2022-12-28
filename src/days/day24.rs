use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day24 {}

impl Day for Day24 {
    fn solve_a(&self, file: &String) -> String {
        let mut blizzards = Blizzards::parse(file);
        
        let start = Point::new(0, -1);
        let end = Point::new(blizzards.max_x - 1, blizzards.max_y);
        
        solve(&mut blizzards, &start, &end, 0).to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let mut blizzards = Blizzards::parse(file);
        
        let start = Point::new(0, -1);
        let end = Point::new(blizzards.max_x - 1, blizzards.max_y);
        
        let fastest_time_1 = solve(&mut blizzards, &start, &end, 0);
        let fastest_time_2 = solve(&mut blizzards, &end, &start, fastest_time_1);
        solve(&mut blizzards, &start, &end, fastest_time_2).to_string()
    }
}

fn solve(blizzards: &mut Blizzards, start_position: &Point, end_position: &Point, start_time: usize) -> usize {
    let mut possible_points: HashSet<Point> = HashSet::from([start_position.clone()]);
    let mut time = start_time;
    
    let directions: Vec<Point> = vec![
        Point::new(1, 0),
        Point::new(-1, 0),
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(0, 0),
    ];
    
    loop {
        time += 1;
        let available_positions = blizzards.get_empty_spaces_at_time(time).clone();
        let mut new_possible_points: HashSet<Point> = HashSet::new();
        
        for point in possible_points.iter() {
            for direction in directions.iter() {
                let new_point = Point::add(point, &direction);
                if new_point.eq(&end_position) {
                    return time;
                }
                if new_point.eq(&start_position) || (blizzards.is_valid_point(&new_point) && available_positions.contains(&new_point)) {
                    new_possible_points.insert(new_point);
                }
            }
        }
        
        possible_points = new_possible_points;
    }
}

#[derive(Debug)]
struct Blizzards {
    clear_spaces_at_time: HashMap<usize, HashSet<Point>>,
    latest_blizzards: Vec<Blizzard>,
    max_x: isize,
    max_y: isize,
}

impl Blizzards {
    fn parse(file: &str) -> Self {
        let mut blizzards: Vec<Blizzard> = vec![];
        let max_x = file.lines().collect::<Vec<&str>>().get(0).unwrap().len() - 2;
        let max_y = file.lines().collect::<Vec<&str>>().len() - 2;
        
        let mut clear_points: HashSet<Point> = HashSet::new();
        
        for (y, line) in file.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if y == 0 || x == 0 || x > max_x || y > max_y || c.eq(&'#') {
                    continue;
                }
                let point = Point::new_usize(x - 1, y - 1);
                if c.eq(&'.') {
                    clear_points.insert(point);
                } else {
                    blizzards.push(Blizzard {
                        location: point,
                        direction: c,
                    });
                }
            }
        }
        
        Blizzards {
            clear_spaces_at_time: HashMap::from([(0, clear_points)]),
            latest_blizzards: blizzards,
            max_x: max_x as isize,
            max_y: max_y as isize,
        }
    }
    
    fn latest_clear_point_set(&self) -> HashSet<Point> {
        let blizzard_points: HashSet<Point> = self.latest_blizzards.iter().map(|b| b.location).collect();
        
        let mut clear_points: HashSet<Point> = HashSet::new();
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let point = Point::new(x, y);
                if !blizzard_points.contains(&point) {
                    clear_points.insert(point);
                }
            }
        }
        clear_points
    }
    
    fn simulate_timestep(&mut self) {
        let directions: HashMap<char, Point> = HashMap::from([
            ('>', Point::new(1, 0)),
            ('v', Point::new(0, 1)),
            ('<', Point::new(-1, 0)),
            ('^', Point::new(0, -1)),
        ]);
        
        let mut new_blizzards: Vec<Blizzard> = vec![];
        
        for blizzard in self.latest_blizzards.iter() {
            let new_position = Point::add(&blizzard.location, directions.get(&blizzard.direction).unwrap()).convert_to_range(self.max_x, self.max_y);
            
            new_blizzards.push(Blizzard {
                location: new_position,
                direction: blizzard.direction,
            });
        }
        
        self.latest_blizzards = new_blizzards;
        self.clear_spaces_at_time.insert(self.latest_time() + 1, self.latest_clear_point_set());
    }
    
    fn get_empty_spaces_at_time(&mut self, time: usize) -> &HashSet<Point> {
        if !self.clear_spaces_at_time.contains_key(&time) {
            self.simulate_timestep();
        }
        self.clear_spaces_at_time.get(&time).unwrap()
    }
    
    fn is_valid_point(&self, point: &Point) -> bool {
        if point.x == 0 && point.y == -1 {
            return true;
        }
        point.x >= 0 && point.x <= self.max_x && point.y >= 0 && point.y <= self.max_y
    }
    
    fn latest_time(&self) -> usize {
        self.clear_spaces_at_time.keys().max().unwrap().clone()
    }
    
    fn _print_latest(&self) {
        self._print_at_time(self.latest_time());
    }
    
    fn _print_at_time(&self, time: usize) {
        println!("\nPrinting blizzards at time: {time}");
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.clear_spaces_at_time.get(&time).unwrap().contains(&Point::new(x, y)) {
                    print!(" ")
                } else {
                    print!("B")
                }
            }
            print!("\n");
        }
        println!("Finished printing\n");
    }
}

#[derive(Debug)]
struct Blizzard {
    location: Point,
    direction: char,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
    
    fn new_usize(x: usize, y: usize) -> Self {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }
    
    fn add(a: &Self, b: &Self) -> Self {
        Point::new(a.x + b.x, a.y + b.y)
    }
    
    fn convert_to_range(&self, max_x: isize, max_y: isize) -> Self {
        Point::new((self.x + max_x) % max_x, (self.y + max_y) % max_y)
    }
}