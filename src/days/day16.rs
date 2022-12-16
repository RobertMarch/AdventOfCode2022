use std::collections::{HashMap, HashSet};

use crate::days::day::Day;

pub struct Day16 {}

impl Day for Day16 {
    fn solve_a(&self, file: &String) -> String {
        let mut pressure_valves: HashMap<String, usize> = HashMap::new();
        let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();
        
        for line in file.lines() {
            let room: String = line[6..8].to_string();
            let (first_part, second_part) = line.split_once("; ").unwrap();
            let flow_rate: usize = first_part[23..].parse().expect("unexpected value");

            if flow_rate > 0 {
                pressure_valves.insert(room.clone(), flow_rate);
            }

            let second_part = second_part.split_once(" to valve").unwrap().1;
            let connections: Vec<String> = second_part[1..].trim().split(", ").map(|s| s.to_string()).collect();
            tunnels.insert(room.clone(), connections);
        }

        let valve_connections: HashMap<String, HashMap<String, usize>> = connect_valves(tunnels, &pressure_valves);
        
        recurse(
            String::from("AA"), 
            0,
            &valve_connections,
            &pressure_valves,
            HashSet::new()
        ).to_string()
    }
    
    fn solve_b(&self, _file: &String) -> String {
        String::from("Not yet implemented")
    }
}

type Valves = HashMap<String, usize>;
type Tunnels = HashMap<String, HashMap<String, usize>>;

fn connect_valves(tunnels: HashMap<String, Vec<String>>, valves: &Valves) -> Tunnels {
    let mut full_connections: Tunnels = HashMap::new();

    let start = String::from("AA");
    full_connections.insert(start.clone(), get_tunnel_distances(&start, &tunnels, &valves));
    
    for valve in valves.keys() {
        full_connections.insert(valve.clone(), get_tunnel_distances(valve, &tunnels, &valves));
    }

    full_connections
}

fn get_tunnel_distances(start: &String, tunnels: &HashMap<String, Vec<String>>, valves: &Valves) -> HashMap<String, usize> {
    let mut known_points: HashMap<String, usize> = HashMap::new();
    let mut unvisited_valves: HashMap<String, usize> = HashMap::new();
    
    for neighbour in tunnels.get(start).unwrap() {
        unvisited_valves.insert(neighbour.clone(), 1);
    }
    
    while unvisited_valves.len() > 0 {
        let vals_for_next = unvisited_valves.clone();
        let next = vals_for_next.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().clone();

        known_points.insert(next.0.clone(), next.1.clone());
        unvisited_valves.remove(next.0);

        for neighbour in tunnels.get(next.0).unwrap() {
            if !known_points.contains_key(neighbour) && 
                (!unvisited_valves.contains_key(neighbour) || unvisited_valves.get(neighbour).unwrap() > &(next.1 + 1)) {
                unvisited_valves.insert(neighbour.clone(), next.1 + 1);
            }
        }    
    }

    let mut connections: HashMap<String, usize> = HashMap::new();

    for valve in known_points.keys() {
        if valves.contains_key(valve) {
            connections.insert(valve.clone(), known_points.get(valve).unwrap().clone());
        }
    }

    connections
}

fn recurse(current: String, time: usize, tunnels: &Tunnels, valves: &Valves, visited_valves: HashSet<String>) -> usize {
    if time >= 30 {
        return 0;
    }

    let mut new_time = time;
    let mut released_pressure = 0;
    
    if valves.contains_key(&current) {
        new_time = time + 1;
        released_pressure = (30 - new_time) * valves.get(&current).unwrap();
    }
    
    let mut options: Vec<usize> = vec![0];
    
    for new_valve in tunnels.get(&current).unwrap().keys() {
        if visited_valves.contains(new_valve) {
            continue
        }

        let mut new_visited = visited_valves.clone();
        new_visited.insert(new_valve.clone());

        options.push(recurse(
            new_valve.clone(),
            new_time + tunnels.get(&current).unwrap().get(new_valve).unwrap(),
            &tunnels.clone(),
            &valves.clone(),
            new_visited
        ));
    }
    
    released_pressure + options.iter().max().unwrap().clone()
}

