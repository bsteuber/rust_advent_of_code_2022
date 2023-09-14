use std::ops::Add;

use crate::util;

type Coord = i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: Coord,
    y: Coord,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Debug)]
struct Rock {
    points: Vec<Point>,
}

impl Rock {
    fn parse(lines: &Vec<String>) -> Self {
        let mut points = vec![];
        for (y, line) in lines.iter().rev().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    points.push(Point {
                        x: x as Coord,
                        y: y as Coord,
                    })
                }
            }
        }
        Self { points }
    }

    fn raw_move(&mut self, direction: Point) {
        for point in &mut self.points {
            *point = *point + direction
        }
    }
}

const WIDTH: Coord = 7;

struct Chamber {
    rocks: Vec<Rock>,
    rock_index: usize,
    directions: Vec<Point>,
    direction_index: usize,
    heighest: Coord,
    occupied: Vec<Vec<bool>>,
    directions_history: Vec<usize>,
    growth_history: Vec<Coord>,
}

impl Chamber {
    fn parse(rock_file: &str, input_file: &str) -> Self {
        let rocks: Vec<Rock> = util::read_blocks(rock_file)
            .iter()
            .map(Rock::parse)
            .collect();
        let directions: Vec<Point> = util::read_str(input_file)
            .chars()
            .map(|c| match c {
                '<' => Point { x: -1, y: 0 },
                '>' => Point { x: 1, y: 0 },
                _ => panic!("Illegal direction: {}", c),
            })
            .collect();
        Self {
            rocks,
            rock_index: 0,
            directions,
            direction_index: 0,
            heighest: -1,
            occupied: Vec::new(),
            directions_history: Vec::new(),
            growth_history: Vec::new(),
        }
    }

    fn next_rock(&mut self) -> Rock {
        let mut rock = self.rocks[self.rock_index].clone();
        rock.raw_move(Point {
            x: 2,
            y: self.heighest + 4,
        });
        self.rock_index = (self.rock_index + 1) % self.rocks.len();
        rock
    }

    fn next_direction(&mut self) -> Point {
        let direction = self.directions[self.direction_index].clone();
        self.direction_index = (self.direction_index + 1) % self.directions.len();
        direction
    }

    fn try_move(&self, rock: &mut Rock, direction: Point) -> bool {
        // println!("Try move {:?}", direction);
        let mut next_rock = rock.clone();
        next_rock.raw_move(direction);
        for p in &next_rock.points {
            if p.x < 0 || p.y < 0 || p.x >= WIDTH {
                // println!("Out of field!");
                return false;
            } else if let Some(row) = self.occupied.get(p.y as usize) {
                if row[p.x as usize] {
                    return false;
                }
            }
        }
        *rock = next_rock;
        return true;
    }

    fn drop_rock(&mut self) {
        let mut rock = self.next_rock();
        loop {
            // println!("\nBefore side");
            // self.print(Some(&rock));
            let dir = self.next_direction();
            self.try_move(&mut rock, dir);
            // self.print(Some(&rock));
            if !self.try_move(&mut rock, Point { x: 0, y: -1 }) {
                let prev_height = self.heighest;
                for p in &rock.points {
                    if p.y as usize == self.occupied.len() {
                        self.occupied.push(vec![false; WIDTH as usize]);
                    }
                    self.occupied[p.y as usize][p.x as usize] = true;
                    if p.y > self.heighest {
                        self.heighest = p.y
                    }
                }
                self.growth_history.push(self.heighest - prev_height);
                self.directions_history.push(self.direction_index);
                return;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self, rock: Option<&Rock>) {
        println!();
        let start_y = if let Some(rock) = rock {
            rock.points.iter().map(|p| p.y).max().unwrap()
        } else {
            self.heighest
        };
        for y in (0..start_y + 1).rev() {
            for x in 0..WIDTH {
                let on_rock = if let Some(rock) = rock {
                    rock.points.iter().find(|p| p.x == x && p.y == y).is_some()
                } else {
                    false
                };
                let on_occupied = if let Some(row) = self.occupied.get(y as usize) {
                    row[x as usize]
                } else {
                    false
                };
                if on_rock {
                    print!("@")
                } else if on_occupied {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    fn check_for_cycle(&self) -> Option<usize> {
        let total_rocks = self.growth_history.len();
        let rock_cycle_len = self.rocks.len();
        let compare_seq_len = 20 * rock_cycle_len;
        if self.rock_index == 0 && total_rocks >= compare_seq_len + rock_cycle_len {
            let last_sequence_start = total_rocks - compare_seq_len;
            let last_growth_sequence = &self.growth_history[last_sequence_start..total_rocks];
            let last_directions_sequence =
                &self.directions_history[last_sequence_start..total_rocks];
            let mut sequence_start = last_sequence_start - rock_cycle_len;
            loop {
                if &self.growth_history[sequence_start..sequence_start + compare_seq_len]
                    == last_growth_sequence
                    && &self.directions_history[sequence_start..sequence_start + compare_seq_len]
                        == last_directions_sequence
                {
                    return Some(last_sequence_start - sequence_start);
                } else if sequence_start < rock_cycle_len {
                    break;
                } else {
                    sequence_start -= rock_cycle_len
                }
            }
        }
        None
    }
}

pub fn run() {
    // let mut chamber = Chamber::parse("17-rocks", "17-test");
    let mut chamber = Chamber::parse("17-rocks", "17-input");
    for _ in 0..2022 {
        chamber.drop_rock();
        // chamber.print(None);
    }
    println!("Part 1: {}", chamber.heighest + 1);

    let cycle_len;

    loop {
        // if chamber.growth_history.len() % chamber.ro {
        //     return;
        // }
        if let Some(len) = chamber.check_for_cycle() {
            cycle_len = len;
            break;
        }
        chamber.drop_rock();
    }

    let rocks_to_drop = 1000000000000usize;
    let mut dropped_rocks = chamber.growth_history.len();
    let cycle = &chamber.growth_history[dropped_rocks - cycle_len..dropped_rocks];
    let cycle_height = cycle.iter().sum::<i32>() as u64;
    let mut height = (chamber.heighest + 1) as u64;
    let left_cycles = (rocks_to_drop - dropped_rocks) / cycle_len;
    dropped_rocks += cycle_len * left_cycles;
    height += cycle_height * left_cycles as u64;
    let offset = dropped_rocks % cycle_len;
    // println!("offset {}", offset);
    while dropped_rocks < rocks_to_drop {
        height += cycle[(dropped_rocks - offset) % cycle_len] as u64;
        dropped_rocks += 1;
    }
    println!("Part 2: {}", height);
}
