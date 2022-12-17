use std::collections::{BTreeSet, HashMap, HashSet};

use crate::days::day::Day;

pub struct Day16 {}

impl Day for Day16 {
    fn solve_a(&self, file: &String) -> String {
        let tunnel_system = TunnelSystem::load(file);

        let mut route_options: HashMap<String, isize> = HashMap::new();
        route_options.insert(String::from("AA"), 0);

        map_all_routes(
            &mut vec![String::from("AA")],
            30,
            &tunnel_system,
            &mut HashSet::new(),
            &mut route_options,
        );

        route_options.values().max().unwrap().to_string()
    }

    fn solve_b(&self, file: &String) -> String {
        let tunnel_system = TunnelSystem::load(file);

        let mut route_options: HashMap<String, isize> = HashMap::new();
        route_options.insert(String::from("AA"), 0);

        map_all_routes(
            &mut vec![String::from("AA")],
            26,
            &tunnel_system,
            &mut HashSet::new(),
            &mut route_options,
        );

        println!("{} routes mapped", route_options.len());

        let mut routes: HashMap<BTreeSet<String>, isize> = HashMap::new();

        for (route, route_pressure) in route_options.iter() {
            if route.eq("AA") {
                continue;
            }
            let nodes: BTreeSet<String> = route[3..].split("/").map(|s| s.to_string()).collect();

            if !routes.contains_key(&nodes) || routes.get(&nodes).unwrap() < route_pressure {
                routes.insert(nodes, route_pressure.clone());
            }
        }

        println!(
            "{} route node combinations from {} mapped routes",
            routes.len(),
            route_options.len()
        );

        let mut best: isize = 0;
        for (route, route_pressure) in routes.iter() {
            for (route_2, route_2_pressure) in routes.iter() {
                if route_2.is_disjoint(&route) {
                    let total = route_pressure + route_2_pressure;

                    if total > best {
                        best = total;
                    }
                }
            }
        }

        best.to_string()
    }
}

type Valves = HashMap<String, isize>;
type Tunnels = HashMap<String, HashMap<String, isize>>;

struct TunnelSystem {
    valves: Valves,
    tunnels: Tunnels,
}

impl TunnelSystem {
    fn load(input: &String) -> Self {
        let mut pressure_valves: HashMap<String, isize> = HashMap::new();
        let mut tunnels: HashMap<String, Vec<String>> = HashMap::new();

        for line in input.lines() {
            let room: String = line[6..8].to_string();
            let (first_part, second_part) = line.split_once("; ").unwrap();
            let flow_rate: isize = first_part[23..].parse().expect("unexpected value");

            if flow_rate > 0 {
                pressure_valves.insert(room.clone(), flow_rate);
            }

            let second_part = second_part.split_once(" to valve").unwrap().1;
            let connections: Vec<String> = second_part[1..]
                .trim()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            tunnels.insert(room.clone(), connections);
        }

        let valve_connections: HashMap<String, HashMap<String, isize>> =
            TunnelSystem::connect_valves(tunnels, &pressure_valves);

        TunnelSystem {
            valves: pressure_valves,
            tunnels: valve_connections,
        }
    }

    fn connect_valves(tunnels: HashMap<String, Vec<String>>, valves: &Valves) -> Tunnels {
        let mut full_connections: Tunnels = HashMap::new();

        let start = String::from("AA");
        full_connections.insert(
            start.clone(),
            TunnelSystem::get_tunnel_distances(&start, &tunnels, &valves),
        );

        for valve in valves.keys() {
            full_connections.insert(
                valve.clone(),
                TunnelSystem::get_tunnel_distances(valve, &tunnels, &valves),
            );
        }

        full_connections
    }

    fn get_tunnel_distances(
        start: &String,
        tunnels: &HashMap<String, Vec<String>>,
        valves: &Valves,
    ) -> HashMap<String, isize> {
        let mut known_points: HashMap<String, isize> = HashMap::new();
        let mut unvisited_valves: HashMap<String, isize> = HashMap::new();

        for neighbour in tunnels.get(start).unwrap() {
            unvisited_valves.insert(neighbour.clone(), 1);
        }

        while unvisited_valves.len() > 0 {
            let vals_for_next = unvisited_valves.clone();
            let next = vals_for_next
                .iter()
                .min_by(|a, b| a.1.cmp(&b.1))
                .unwrap()
                .clone();

            known_points.insert(next.0.clone(), next.1.clone());
            unvisited_valves.remove(next.0);

            for neighbour in tunnels.get(next.0).unwrap() {
                if !known_points.contains_key(neighbour)
                    && (!unvisited_valves.contains_key(neighbour)
                        || unvisited_valves.get(neighbour).unwrap() > &(next.1 + 1))
                {
                    unvisited_valves.insert(neighbour.clone(), next.1 + 1);
                }
            }
        }

        let mut connections: HashMap<String, isize> = HashMap::new();

        for valve in known_points.keys() {
            if valves.contains_key(valve) {
                connections.insert(valve.clone(), known_points.get(valve).unwrap().clone());
            }
        }

        connections
    }
}

fn map_all_routes(
    current_path: &mut Vec<String>,
    time_remaining: isize,
    tunnel_system: &TunnelSystem,
    visited_valves: &mut HashSet<String>,
    routes: &mut HashMap<String, isize>,
) {
    if time_remaining <= 0 {
        return;
    }

    let current_valve = current_path.last().unwrap().clone();
    let mut new_time_remaining = time_remaining;

    if tunnel_system.valves.contains_key(&current_valve) {
        new_time_remaining = time_remaining - 1;
        let released_pressure =
            new_time_remaining * tunnel_system.valves.get(&current_valve).unwrap();

        routes.insert(
            current_path.join("/"),
            routes
                .get(&current_path[..current_path.len() - 1].join("/"))
                .unwrap()
                + released_pressure,
        );
    }

    for new_valve in tunnel_system.tunnels.get(&current_valve).unwrap().keys() {
        if visited_valves.contains(new_valve) {
            continue;
        }

        visited_valves.insert(new_valve.clone());
        current_path.push(new_valve.clone());

        map_all_routes(
            current_path,
            new_time_remaining
                - tunnel_system
                    .tunnels
                    .get(&current_valve)
                    .unwrap()
                    .get(new_valve)
                    .unwrap()
                    .clone() as isize,
            tunnel_system,
            visited_valves,
            routes,
        );

        visited_valves.remove(new_valve);
        current_path.pop();
    }
}
