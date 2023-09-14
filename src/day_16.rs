use crate::util;

#[derive(Debug)]
struct CaveSystem {
    valve_names: Vec<String>,
    rates: Vec<usize>,
    links: Vec<Vec<usize>>,
    shortest_path: Vec<Vec<usize>>,
    count: usize,
}

const MINUTES: usize = 30;

impl CaveSystem {
    fn get_index(&mut self, valve_name: &str) -> usize {
        if let Some((i, _)) = self
            .valve_names
            .iter()
            .enumerate()
            .find(|(_index, name)| *name == valve_name)
        {
            i
        } else {
            self.valve_names.push(valve_name.to_string());
            self.rates.push(0);
            self.links.push(Vec::new());
            self.count += 1;
            self.count - 1
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
            rates: Vec::new(),
            links: Vec::new(),
            shortest_path: Vec::new(),
            count: 0,
        };

        system.get_index("AA");

        for line in util::read_lines(file) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let valve = system.get_index(tokens[1]);
            let rate = tokens[4]
                .replace("rate=", "")
                .replace(";", "")
                .parse()
                .expect("Failed to parse rate");
            system.rates[valve] = rate;
            for linked_valve in &tokens[9..] {
                let target = system.get_index(&linked_valve.replace(",", ""));
                system.links[valve].push(target);
            }
            system.shortest_path = vec![vec![9999; system.count]; system.count];
            for source in 0..system.count {
                system.find_shortest_paths(source)
            }
        }
        system
    }

    #[allow(dead_code)]
    fn print_shortest_paths(&self) {
        for source in 0..self.count {
            let src = &self.valve_names[source];
            for target in 0..self.count {
                let tgt = &self.valve_names[target];
                println!(
                    "{} => {}\t{} min",
                    src, tgt, self.shortest_path[source][target],
                );
            }
        }
    }

    fn find_max_flow(&self) -> usize {
        let mut max_flow = 0;
        let mut is_open = vec![false; self.count];
        let mut history = vec![];
        self.rec_find_max_flow(&mut max_flow, 0, &mut is_open, 0, 0, 0, &mut history);
        max_flow
    }

    fn find_max_two_threads(&self) -> usize {
        let mut max_flow = 0;
        let mut is_open = vec![false; self.count];
        let mut history = vec![];
        self.rec_find_max_flow_two_threads(
            &mut max_flow,
            0,
            &mut is_open,
            0,
            0,
            0,
            &mut history,
            None,
        );
        max_flow
    }

    #[allow(dead_code)]
    fn is_along_path(&self, history: &[usize], path: &[&str]) -> bool {
        (0..history.len()).all(|i| {
            if let Some(valve) = path.get(i) {
                self.valve_names[history[i]] == *valve
            } else {
                false
            }
        })
    }

    #[allow(dead_code)]
    fn prn_history(&self, history: &[usize]) {
        for valve in history {
            print!("{} ", self.valve_names[*valve]);
        }
        println!("")
    }

    fn rec_find_max_flow(
        &self,
        max_flow: &mut usize,
        minute: usize,
        is_open: &mut Vec<bool>,
        current: usize,
        total_flow: usize,
        current_flow: usize,
        history: &mut Vec<usize>,
    ) {
        let left = (0..self.count)
            .filter(|x| !is_open[*x] && self.rates[*x] > 0)
            .count();
        let (total_flow, minute) = if left == 0 {
            (total_flow + current_flow * (MINUTES - minute), MINUTES)
        } else {
            (total_flow, minute)
        };

        if minute == MINUTES {
            if total_flow > *max_flow {
                *max_flow = total_flow;
            }
            return;
        }
        for next in 0..self.count {
            if (!is_open[next]) && self.rates[next] > 0 {
                let min_passed = self.shortest_path[current][next] + 1;
                let next_minute = MINUTES.min(minute + min_passed);
                let min_passed = next_minute - minute;
                let total_flow = total_flow + min_passed * current_flow;
                history.push(next);
                let current_flow = current_flow + self.rates[next];
                is_open[next] = true;
                self.rec_find_max_flow(
                    max_flow,
                    next_minute,
                    is_open,
                    next,
                    total_flow,
                    current_flow,
                    history,
                );
                history.pop();
                is_open[next] = false;
            }
        }
    }

    fn rec_find_max_flow_two_treads(
        &self,
        max_flow: &mut usize,
        minute: usize,
        is_open: &mut Vec<bool>,
        current: usize,
        total_flow: usize,
        current_flow: usize,
        history: &mut Vec<usize>,
        pending: Option<(usize, usize)>,
    ) {
        let left = (0..self.count)
            .filter(|x| !is_open[*x] && self.rates[*x] > 0)
            .count();
        let (total_flow, minute) = if left == 0 {
            (total_flow + current_flow * (MINUTES - minute), MINUTES)
        } else {
            (total_flow, minute)
        };

        if minute == MINUTES {
            if total_flow > *max_flow {
                *max_flow = total_flow;
            }
            return;
        }
        for next in 0..self.count {
            if (!is_open[next]) && self.rates[next] > 0 {
                let min_passed = self.shortest_path[current][next] + 1;
                let next_minute = MINUTES.min(minute + min_passed);
                let min_passed = next_minute - minute;
                let total_flow = total_flow + min_passed * current_flow;
                history.push(next);
                let current_flow = current_flow + self.rates[next];
                is_open[next] = true;
                self.rec_find_max_flow(
                    max_flow,
                    next_minute,
                    is_open,
                    next,
                    total_flow,
                    current_flow,
                    history,
                );
                history.pop();
                is_open[next] = false;
            }
        }
    }
}

fn part_1(system: &CaveSystem) {
    println!("Part 1: {}", system.find_max_flow());
}

pub fn run() {
    let test = CaveSystem::parse("16-test");
    let input = CaveSystem::parse("16-input");
    part_1(&test);
    part_1(&input);
}
