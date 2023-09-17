use std::collections::HashMap;

use crate::util;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct SearchState {
    time_remaining: usize,
    is_open: Vec<bool>,
    current_valve: usize,
}

#[derive(Debug)]
struct CaveSystem {
    valve_names: Vec<String>,
    flow_rates: Vec<usize>,
    links: Vec<Vec<usize>>,
    shortest_path: Vec<Vec<usize>>,
    valve_count: usize,
    cache: HashMap<SearchState, usize>,
    // debug: HashMap<usize, usize>,
}

impl CaveSystem {
    fn valve_index(&mut self, valve_name: &str) -> usize {
        if let Some((i, _)) = self
            .valve_names
            .iter()
            .enumerate()
            .find(|(_index, name)| *name == valve_name)
        {
            i
        } else {
            self.valve_names.push(valve_name.to_string());
            self.flow_rates.push(0);
            self.links.push(Vec::new());
            self.valve_count += 1;
            self.valve_count - 1
        }
    }

    fn find_shortest_paths(&mut self, source: usize) {
        self.shortest_path[source][source] = 0;
        self.rec_find_path(0, source, source);
    }

    fn rec_find_path(&mut self, len: usize, source: usize, current: usize) {
        let len = len + 1;
        let next_items = self.links[current].clone();
        for next in &next_items {
            if len < self.shortest_path[source][*next] {
                self.shortest_path[source][*next] = len;
                self.rec_find_path(len, source, *next);
            }
        }
    }

    fn parse(file: &str) -> Self {
        let mut system = Self {
            valve_names: Vec::new(),
            flow_rates: Vec::new(),
            links: Vec::new(),
            shortest_path: Vec::new(),
            valve_count: 0,
            cache: HashMap::new(),
            // debug: HashMap::new(),
        };

        system.valve_index("AA");

        for line in util::read_lines(file) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let valve = system.valve_index(tokens[1]);
            let rate = tokens[4]
                .replace("rate=", "")
                .replace(";", "")
                .parse()
                .expect("Failed to parse rate");
            system.flow_rates[valve] = rate;
            for linked_valve in &tokens[9..] {
                let target = system.valve_index(&linked_valve.replace(",", ""));
                system.links[valve].push(target);
            }
            system.shortest_path = vec![vec![9999; system.valve_count]; system.valve_count];
            for source in 0..system.valve_count {
                system.find_shortest_paths(source)
            }
        }
        system
    }

    #[allow(dead_code)]
    fn print_shortest_paths(&self) {
        for source in 0..self.valve_count {
            let src = &self.valve_names[source];
            for target in 0..self.valve_count {
                let tgt = &self.valve_names[target];
                println!(
                    "{} => {}\t{} min",
                    src, tgt, self.shortest_path[source][target],
                );
            }
        }
    }

    fn find_max_flow(&mut self, time_limit: usize, limit_valves: Option<Vec<usize>>) -> usize {
        let is_open = match limit_valves {
            None => vec![false; self.valve_count],
            Some(valves) => {
                let mut is_open = vec![true; self.valve_count];
                for valve in valves {
                    is_open[valve] = false;
                }
                is_open
            }
        };

        let start_valve = self.valve_index("AA");
        let root = SearchState {
            time_remaining: time_limit,
            is_open: is_open,
            current_valve: start_valve,
        };
        self.rec_find_max_flow(root)
    }

    fn rec_find_max_flow(&mut self, state: SearchState) -> usize {
        if let Some(value) = self.cache.get(&state) {
            return *value;
        }

        let mut max_flow = 0;

        for valve in 0..self.valve_count {
            // if !self.debug.is_empty() {
            //     match self.debug.get(&(&state.time_remaining)) {
            //         None => continue,
            //         Some(v) => {
            //             if *v != valve {
            //                 continue;
            //             }
            //         }
            //     }
            // }
            if !state.is_open[valve] && self.flow_rates[valve] > 0 {
                let travel_time = self.shortest_path[state.current_valve][valve] + 1;
                if state.time_remaining > travel_time {
                    let time_remaining = state.time_remaining - travel_time;
                    let mut state = state.clone();
                    state.current_valve = valve;
                    state.is_open[valve] = true;
                    state.time_remaining = time_remaining;
                    let produced_flow = self.flow_rates[valve] * time_remaining;
                    // println!(
                    //     "Opened {} and produced  {} x {} = {}",
                    //     valve, time_remaining, self.flow_rates[valve], produced_flow
                    // );
                    let flow = produced_flow + self.rec_find_max_flow(state);
                    // println!("Total flow: {}", flow);
                    if flow > max_flow {
                        max_flow = flow;
                    }
                }
            }
        }
        self.cache.insert(state, max_flow);
        return max_flow;
    }

    fn calc_partitions(&mut self) -> Vec<(Vec<usize>, Vec<usize>)> {
        let mut valves: Vec<usize> = (0..self.valve_count)
            .filter(|valve| self.flow_rates[*valve] > 0)
            .collect();
        let mut possible_human_valves = vec![vec![valves.pop().unwrap()]];
        for valve in &valves {
            for human_valves_index in 0..possible_human_valves.len() {
                let mut with_valve = possible_human_valves[human_valves_index].clone();
                with_valve.push(*valve);
                possible_human_valves.push(with_valve);
            }
        }
        let mut ret = vec![];
        for human_valves in possible_human_valves {
            let mut elephant_valves = vec![];
            for valve in &valves {
                if !human_valves.contains(valve) {
                    elephant_valves.push(*valve);
                }
            }
            ret.push((human_valves, elephant_valves));
        }
        ret
    }
}

fn part_1(file: &str) {
    let mut system = CaveSystem::parse(file);
    println!("Part 1: {}", system.find_max_flow(30, None));
}

fn part_2(file: &str) {
    let mut system = CaveSystem::parse(file);
    let partitions = system.calc_partitions();
    let mut best = 0;
    let partition_count = partitions.len();
    println!("Partitions to do: {}", partition_count);
    let mut i = 0;
    for (human_valves, elephant_valves) in partitions {
        i += 1;
        if i % 100 == 0 {
            println!("Done {} / {}", i, partition_count);
        }
        let human_flow = system.find_max_flow(26, Some(human_valves));
        let elephant_flow = system.find_max_flow(26, Some(elephant_valves));
        let total = human_flow + elephant_flow;
        if total > best {
            best = total;
        }
    }
    println!("Part 2: {}", best);
}

pub fn run() {
    let file = "16-input";
    part_1(file);
    part_2(file);
}
