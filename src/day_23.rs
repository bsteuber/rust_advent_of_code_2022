use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::util;

type Num = i32;
type Point = (Num, Num);

struct State {
    elves: Vec<Point>,
    occupied: HashSet<Point>,
    consider_offset: usize,
}

const N: Point = (0, -1);
const NW: Point = (-1, -1);
const NE: Point = (1, -1);
const S: Point = (0, 1);
const SW: Point = (-1, 1);
const SE: Point = (1, 1);
const W: Point = (-1, 0);
const E: Point = (1, 0);

const ALL_DIRECTIONS: [Point; 8] = [N, NW, NE, S, SE, SW, E, W];

const CONSIDER_DELTAS: [[Point; 3]; 4] = [[N, NW, NE], [S, SE, SW], [W, NW, SW], [E, NE, SE]];

enum ProposalStatus {
    Elf(usize),
    Blocked,
}

impl State {
    fn parse(file: &str) -> Self {
        let mut elves = vec![];
        let mut occupied = HashSet::new();
        for (y, line) in util::read_lines(file).iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.push((x as Num, y as Num));
                    occupied.insert((x as Num, y as Num));
                }
            }
        }
        Self {
            elves,
            occupied,
            consider_offset: 0,
        }
    }

    fn neighbour(point: &Point, delta: &Point) -> Point {
        (point.0 + delta.0, point.1 + delta.1)
    }

    fn consider_direction(&self, point: &Point, check_deltas: &[Point; 3]) -> Option<Point> {
        for delta in check_deltas {
            let neighb = Self::neighbour(point, delta);
            if self.occupied.contains(&neighb) {
                return None;
            }
        }
        return Some(Self::neighbour(point, &check_deltas[0]));
    }

    fn does_consider(&self, point: &Point) -> bool {
        ALL_DIRECTIONS
            .iter()
            .find(|delta| self.occupied.contains(&Self::neighbour(point, delta)))
            .is_some()
    }

    fn consider(&self, point: &Point) -> Option<Point> {
        for i in 0..4 {
            let index = (i + self.consider_offset) % 4;

            if let Some(proposed) = self.consider_direction(point, &CONSIDER_DELTAS[index]) {
                return Some(proposed);
            }
        }
        None
    }

    fn move_elf(&mut self, id: usize, point: &Point) {
        self.occupied.remove(&self.elves[id]);
        self.occupied.insert(*point);
        self.elves[id] = *point;
    }

    fn make_turn(&mut self) -> bool {
        let mut proposals = HashMap::new();
        for (id, point) in self.elves.iter().enumerate() {
            if self.does_consider(point) {
                if let Some(proposal) = self.consider(point) {
                    let status = if proposals.contains_key(&proposal) {
                        ProposalStatus::Blocked
                    } else {
                        ProposalStatus::Elf(id)
                    };
                    proposals.insert(proposal, status);
                }
            }
        }
        let mut moved_any = false;
        for (proposal, status) in proposals.iter() {
            if let ProposalStatus::Elf(elf) = status {
                self.move_elf(*elf, proposal);
                moved_any = true;
            }
        }
        self.consider_offset += 1;
        moved_any
    }

    fn bounding_box(&self) -> (Point, Point) {
        let mut min_x = 99999999;
        let mut min_y = 99999999;
        let mut max_x = -99999999;
        let mut max_y = -99999999;
        for (x, y) in &self.elves {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }
        ((min_x, min_y), (max_x, max_y))
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((min_x, min_y), (max_x, max_y)) = self.bounding_box();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.occupied.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part_1(file: &str) -> usize {
    let mut state = State::parse(file);
    for _ in 0..10 {
        state.make_turn();
    }
    let ((min_x, min_y), (max_x, max_y)) = state.bounding_box();
    let points = (max_x + 1 - min_x) * (max_y + 1 - min_y);
    points as usize - state.elves.len()
}

fn part_2(file: &str) -> usize {
    let mut state = State::parse(file);
    for i in 1.. {
        if !state.make_turn() {
            return i;
        }
    }
    return 0;
}

pub fn run() {
    println!("Part 1: {}", part_1("23-input"));
    println!("Part 2: {}", part_2("23-input"))
}
