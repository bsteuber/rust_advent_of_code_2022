use std::collections::HashSet;

use crate::util;

type Point = (usize, usize);

enum Direction {
    Down,
    Right,
    Left,
    Up,
}

struct Blizzard {
    position: Point,
    direction: Direction,
}

struct Solver {
    width: usize,
    height: usize,
    start: Point,
    goal: Point,
    blizzards: Vec<Blizzard>,
    time: usize,
    possible_positions: HashSet<Point>,
}

impl Solver {
    fn calc_occupied(&mut self) -> HashSet<Point> {
        self.blizzards.iter().map(|b| b.position).collect()
    }

    fn move_blizzards(&mut self) {
        for blizzard in &mut self.blizzards {
            match blizzard.direction {
                Direction::Up => {
                    blizzard.position.1 = if blizzard.position.1 > 1 {
                        blizzard.position.1 - 1
                    } else {
                        self.height - 2
                    }
                }
                Direction::Left => {
                    blizzard.position.0 = if blizzard.position.0 > 1 {
                        blizzard.position.0 - 1
                    } else {
                        self.width - 2
                    }
                }
                Direction::Down => {
                    blizzard.position.1 = if blizzard.position.1 + 2 < self.height {
                        blizzard.position.1 + 1
                    } else {
                        1
                    }
                }
                Direction::Right => {
                    blizzard.position.0 = if blizzard.position.0 + 2 < self.width {
                        blizzard.position.0 + 1
                    } else {
                        1
                    }
                }
            }
        }
    }

    fn parse(file: &str) -> Self {
        let lines = util::read_lines(file);
        let width = lines[0].len();
        let height = lines.len();
        let start = (1, 0);
        let goal = (width - 2, height - 1);
        let mut blizzards = vec![];
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x, y);
                match c {
                    '^' => blizzards.push(Blizzard {
                        position: pos,
                        direction: Direction::Up,
                    }),
                    '<' => blizzards.push(Blizzard {
                        position: pos,
                        direction: Direction::Left,
                    }),
                    '>' => blizzards.push(Blizzard {
                        position: pos,
                        direction: Direction::Right,
                    }),
                    'v' => blizzards.push(Blizzard {
                        position: pos,
                        direction: Direction::Down,
                    }),
                    _ => (),
                }
            }
        }
        let mut possible_positions = HashSet::new();
        possible_positions.insert(start);
        let mut solver = Self {
            width,
            height,
            start,
            goal,
            blizzards,
            time: 0,
            possible_positions,
        };
        solver.calc_occupied();
        solver
    }

    #[allow(dead_code)]
    fn print_state(&mut self) {
        let width = self.width;
        let height = self.height;
        let start = self.start;
        let goal = self.goal;
        let occupied = self.calc_occupied();
        println!("\nTime: {}", self.time);
        for y in 0..height {
            for x in 0..width {
                let point = (x, y);
                if (x == 0 || y == 0 || x + 1 == width || y + 1 == height)
                    && point != start
                    && point != goal
                {
                    print!("#");
                } else if occupied.contains(&point) {
                    print!("X");
                } else if self.possible_positions.contains(&point) {
                    print!("E")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
    fn run(&mut self, goal: &Point) -> usize {
        // self.print_state();

        for _ in 0.. {
            self.time += 1;
            self.move_blizzards();
            let occupied = self.calc_occupied();
            let mut next_possible = HashSet::new();
            let mut maybe_insert = |p: Point| {
                let on_wall =
                    (p.0 == 0 || p.1 == 0 || p.0 + 1 == self.width || p.1 + 1 == self.height)
                        && p != self.start
                        && p != self.goal;
                if !on_wall && !occupied.contains(&p) {
                    next_possible.insert(p);
                }
            };
            for (x, y) in &self.possible_positions {
                maybe_insert((*x, *y));
                if *x > 0 {
                    maybe_insert((*x - 1, *y));
                }
                if *y > 0 {
                    maybe_insert((*x, *y - 1));
                }

                maybe_insert((*x + 1, *y));

                maybe_insert((*x, *y + 1));
            }
            self.possible_positions = next_possible;
            // self.print_state();
            // println!("Possible: {:#?}", self.possible_positions);
            if self.possible_positions.contains(&goal) {
                return self.time;
            }
        }
        0
    }
}

pub fn run() {
    let mut solver = Solver::parse("24-input");
    let goal = solver.goal;
    let start = solver.start;
    let part_1 = solver.run(&goal);
    println!("Part 1: {}", part_1);
    solver.possible_positions.clear();
    solver.possible_positions.insert(goal);
    solver.run(&start);
    // println!("Back after: {}", back_after);
    solver.possible_positions.clear();
    solver.possible_positions.insert(start);
    let part_2 = solver.run(&goal);
    println!("Part 2: {}", part_2);
}
