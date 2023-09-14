use std::{collections::HashSet, ops};

use crate::util;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn step_towards(self, other: Self) -> Self {
        let diff = other - self;
        let delta = diff.signum();
        let abs = diff.abs();

        if abs.x == 2 || abs.y == 2 {
            self + delta
        } else {
            self
        }
    }
}

struct Rope {
    knots: Vec<Point>,
    tail_set: HashSet<Point>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        let mut tail_set = HashSet::new();
        let mut knots = Vec::new();
        tail_set.insert(Point::origin());
        for _ in 0..knot_count {
            knots.push(Point::origin());
        }
        Self { knots, tail_set }
    }

    fn move_head(&mut self, delta: Point) {
        self.knots[0] = self.knots[0] + delta;
        for i in 1..self.knots.len() {
            self.knots[i] = self.knots[i].step_towards(self.knots[i - 1]);
        }
        self.tail_set.insert(self.knots.last().unwrap().clone());
    }
}

struct Command {
    delta: Point,
    steps: u32,
}

impl Command {
    fn parse(line: &str) -> Self {
        let mut tokens = line.split_whitespace();
        let delta_str = tokens.next().unwrap();
        let steps_str = tokens.next().unwrap();
        let delta = match delta_str {
            "R" => Point::new(1, 0),
            "L" => Point::new(-1, 0),
            "U" => Point::new(0, 1),
            "D" => Point::new(0, -1),
            _ => panic!("Illegal direction"),
        };
        let steps = steps_str.parse().unwrap();
        Self { delta, steps }
    }
}

pub fn part_1(file: &str) -> usize {
    let mut rope = Rope::new(2);
    for line in util::read_lines(file) {
        let Command { delta, steps } = Command::parse(&line);
        for _ in 0..steps {
            rope.move_head(delta);
        }
    }
    rope.tail_set.len()
}

pub fn part_2(file: &str) -> usize {
    let mut rope = Rope::new(10);
    for line in util::read_lines(file) {
        let Command { delta, steps } = Command::parse(&line);
        for _ in 0..steps {
            rope.move_head(delta);
        }
    }
    rope.tail_set.len()
}
