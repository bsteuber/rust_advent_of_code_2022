use std::collections::HashMap;

use crate::util;

type Amount = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Resources {
    items: [Amount; 4],
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const ELEMENTS: usize = 4;

impl Resources {
    fn empty() -> Self {
        Self {
            items: [0; ELEMENTS],
        }
    }

    fn initial_production() -> Self {
        let mut res = Self::empty();
        res.items[ORE] = 1;
        res
    }
}

impl std::ops::Add for Resources {
    type Output = Resources;
    fn add(self, other: Self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..ELEMENTS {
            res.items[i] += other.items[i];
        }
        res
    }
}

impl std::ops::Sub for Resources {
    type Output = Resources;
    fn sub(self, other: Self) -> Self::Output {
        let mut res = self.clone();
        for i in 0..ELEMENTS {
            res.items[i] -= other.items[i];
        }
        res
    }
}

#[derive(Debug)]
struct Action {
    required_resources: Resources,
    production_increase: Resources,
}

impl Action {
    fn new() -> Self {
        Self {
            required_resources: Resources::empty(),
            production_increase: Resources::empty(),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    actions: Vec<Action>,
    max_required: Resources,
}

impl Blueprint {
    fn parse(line: &String) -> Self {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let mut prod_ore = Action::new();
        prod_ore.required_resources.items[ORE] = tokens[6].parse().unwrap();
        prod_ore.production_increase.items[ORE] = 1;
        let mut prod_clay = Action::new();
        prod_clay.required_resources.items[ORE] = tokens[12].parse().unwrap();
        prod_clay.production_increase.items[CLAY] = 1;
        let mut prod_obs = Action::new();
        prod_obs.required_resources.items[ORE] = tokens[18].parse().unwrap();
        prod_obs.required_resources.items[CLAY] = tokens[21].parse().unwrap();
        prod_obs.production_increase.items[OBSIDIAN] = 1;
        let mut prod_geode = Action::new();
        prod_geode.required_resources.items[ORE] = tokens[27].parse().unwrap();
        prod_geode.required_resources.items[OBSIDIAN] = tokens[30].parse().unwrap();
        prod_geode.production_increase.items[GEODE] = 1;
        let actions = vec![prod_ore, prod_clay, prod_obs, prod_geode];
        let mut max_required = Resources::empty();
        for e in 0..ELEMENTS {
            for action in &actions {
                if action.required_resources.items[e] > max_required.items[e] {
                    max_required.items[e] = action.required_resources.items[e];
                }
            }
        }
        Self {
            actions,
            max_required,
        }
    }
    fn parse_all(file: &str) -> Vec<Self> {
        util::read_lines(file).iter().map(Self::parse).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SearchState {
    minute: usize,
    production: Resources,
    resources: Resources,
}

impl SearchState {
    fn new() -> Self {
        Self {
            minute: 0,
            production: Resources::initial_production(),
            resources: Resources::empty(),
        }
    }

    fn can_apply(&self, action: &Action) -> bool {
        for i in 0..ELEMENTS {
            if self.resources.items[i] < action.required_resources.items[i] {
                return false;
            }
        }
        true
    }
    fn apply(&mut self, action: &Action) {
        self.resources = self.resources - action.required_resources;
        self.production = self.production + action.production_increase;
    }
}

struct Search<'a> {
    blueprint: &'a Blueprint,
    cache: HashMap<SearchState, usize>,
    time_available: usize,
}

impl<'a> Search<'a> {
    fn run(blueprint: &'a Blueprint, time_available: usize) -> usize {
        let mut search = Self {
            blueprint,
            cache: HashMap::new(),
            time_available,
        };
        search.find_max_geodes(&SearchState::new())
    }

    fn time_step(&mut self, state: &mut SearchState) -> bool {
        state.minute += 1;
        state.resources = state.resources + state.production;
        if state.minute == self.time_available {
            // println!("Time ended! Geodes: {}", geodes);
            false
        } else {
            true
        }
    }

    fn should_eventually_apply(&self, state: &SearchState, action: &Action) -> bool {
        for i in 0..GEODE {
            if action.required_resources.items[i] > 0 && state.production.items[i] == 0 {
                return false;
            }
            if action.production_increase.items[i] > 0
                && state.production.items[i] == self.blueprint.max_required.items[i]
            {
                return false;
            }
        }
        true
    }

    fn find_max_geodes(&mut self, state: &SearchState) -> usize {
        if let Some(geodes) = self.cache.get(state) {
            return *geodes;
        }
        let mut geodes = 0;
        // println!("Search {}min, {:?} prod", state.minute, state.production);
        'action: for action in &self.blueprint.actions {
            if self.should_eventually_apply(state, action) {
                // println!("Awaiting action {:?}", action.production_increase);
                let mut state = state.clone();
                while !state.can_apply(action) {
                    if !self.time_step(&mut state) {
                        if state.resources.items[GEODE] > geodes {
                            geodes = state.resources.items[GEODE];
                        }
                        continue 'action;
                    }
                }
                if !self.time_step(&mut state) {
                    if state.resources.items[GEODE] > geodes {
                        geodes = state.resources.items[GEODE];
                    }
                    continue 'action;
                }
                state.apply(action);
                let inner = self.find_max_geodes(&state);
                if inner > geodes {
                    geodes = inner
                };
            }
        }
        self.cache.insert(state.clone(), geodes);
        geodes
    }
}

pub fn run() {
    let blueprints = Blueprint::parse_all("19-test");
    // let blueprints = Blueprint::parse_all("19-input");
    let quality_sum: usize = blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| {
            println!("Blueprint {}", i + 1);
            // println!("Max: {:?}", blueprint.max_required);
            let res = Search::run(blueprint, 24);
            println!("Res: {}", res);
            (i + 1) * res
        })
        .sum();
    println!("Part 1: {}", quality_sum);
    let geode_product: usize = blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            println!("Blueprint");
            let res = Search::run(blueprint, 26);
            println!("Res: {}", res);
            res
        })
        .product();
    println!("Part 2: {}", geode_product);
}
