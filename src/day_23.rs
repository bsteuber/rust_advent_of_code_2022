use std::collections::{HashMap, HashSet};

use crate::util;

type Num = i32;
type Point = (Num, Num);

struct State {
    elves: Vec<Point>,
    occupied: HashSet<Point>,
    consider_offset: usize,
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

    fn consider(&mut self, point: Point) -> Option<Point> {
        for i in self.consider_offset..self.consider_offset + 4 {
            match i % 4 {
                0 => todo!(),
                1 => todo!(),
                2 => todo!(),
                3 => todo!(),
                _ => panic!(),
            }
        }
        None
    }

    fn make_turn(&mut self) -> bool {
        todo!()
    }
}

pub fn run() {
    let state = State::parse("23-test-1");
    println!("{:#?}", state.occupied)
}
