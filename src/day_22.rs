use std::collections::HashMap;

use crate::util;

#[derive(Debug, Clone)]
enum Tile {
    Wrap,
    Empty,
    Block,
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            ' ' => Tile::Wrap,
            '.' => Tile::Empty,
            '#' => Tile::Block,
            _ => panic!("Illegal tile char: {}", c),
        }
    }

    fn on_map(&self) -> bool {
        !matches!(self, Tile::Wrap)
    }
}

#[derive(Clone, Debug)]
enum Step {
    Forward,
    Left,
    Right,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }

    fn rotate_left(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Right => *self = Direction::Up,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn to_facing(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

impl Map {
    fn parse(block: &[String]) -> Self {
        let tiles: Vec<Vec<Tile>> = block
            .iter()
            .map(|line| line.chars().map(Tile::parse).collect())
            .collect();
        Self { tiles }
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        row < self.tiles.len() && col < self.tiles[row].len() && self.tiles[row][col].on_map()
    }

    fn first_of_row(&self, row: usize) -> usize {
        (0..self.tiles[row].len())
            .find(|col| self.contains(row, *col))
            .expect("First of row")
    }

    fn last_of_row(&self, row: usize) -> usize {
        (0..self.tiles[row].len())
            .rev()
            .find(|col| self.contains(row, *col))
            .expect("Last of row")
    }

    fn first_of_col(&self, col: usize) -> usize {
        (0..self.tiles.len())
            .find(|row| self.contains(*row, col))
            .expect("First of col")
    }

    fn last_of_col(&self, col: usize) -> usize {
        (0..self.tiles.len())
            .rev()
            .find(|row| self.contains(*row, col))
            .expect("Last of col")
    }

    fn up_of(&self, row: usize, col: usize) -> (usize, usize) {
        if row > 0 && self.contains(row - 1, col) {
            (row - 1, col)
        } else {
            // println!(
            //     "Wrap up of {} -> {} - tile: {:?}",
            //     col,
            //     self.last_of_col(col),
            //     self.tiles[row][self.last_of_col(col)]
            // );
            (self.last_of_col(col), col)
        }
    }

    fn down_of(&self, row: usize, col: usize) -> (usize, usize) {
        if self.contains(row + 1, col) {
            (row + 1, col)
        } else {
            (self.first_of_col(col), col)
        }
    }

    fn left_of(&self, row: usize, col: usize) -> (usize, usize) {
        if col > 0 && self.contains(row, col - 1) {
            (row, col - 1)
        } else {
            (row, self.last_of_row(row))
        }
    }

    fn right_of(&self, row: usize, col: usize) -> (usize, usize) {
        if self.contains(row, col + 1) {
            (row, col + 1)
        } else {
            (row, self.first_of_row(row))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

fn up(row: usize, col: usize) -> Position {
    Position {
        row,
        col,
        direction: Direction::Up,
    }
}

fn down(row: usize, col: usize) -> Position {
    Position {
        row,
        col,
        direction: Direction::Down,
    }
}

fn left(row: usize, col: usize) -> Position {
    Position {
        row,
        col,
        direction: Direction::Left,
    }
}

fn right(row: usize, col: usize) -> Position {
    Position {
        row,
        col,
        direction: Direction::Right,
    }
}

fn link(map: &mut HashMap<Position, Position>, from: Position, to: Position) {
    match map.insert(from, to) {
        Some(_) => panic!("Duplicate cube link source"),
        None => {}
    }
    let back_from = Position {
        direction: to.direction.opposite(),
        ..to
    };
    let back_to = Position {
        direction: from.direction.opposite(),
        ..from
    };
    match map.insert(back_from, back_to) {
        Some(_) => panic!("Duplicate back link source"),
        None => {}
    }
}

fn calc_cube_links(file: &str) -> HashMap<Position, Position> {
    let mut ret = HashMap::new();
    if file == "22-test" {
        let c = 4;
        for p in 0..c {
            let n = c - 1 - p;
            link(&mut ret, up(0, 2 * c + p), down(c, n));
            link(&mut ret, left(p, 2 * c), up(c, c + p));
            link(&mut ret, left(c + p, 0), up(3 * c - 1, 3 * c + n));
            link(&mut ret, down(2 * c - 1, p), up(3 * c - 1, 2 * c + n));
            link(&mut ret, down(2 * c - 1, c + p), right(2 * c + n, 2 * c));
            link(&mut ret, right(p, 3 * c - 1), left(2 * c + n, 4 * c - 1));
            link(&mut ret, right(c + p, 3 * c - 1), down(2 * c, 3 * c + n));
        }
    } else if file == "22-input" {
        let c = 50;
        for p in 0..c {
            let n = c - 1 - p;
            link(&mut ret, up(0, c + p), right(3 * c + p, 0));
            link(&mut ret, up(2 * c, p), right(c + p, c));
            link(&mut ret, left(p, c), right(2 * c + n, 0));
            link(&mut ret, right(3 * c + p, c - 1), up(3 * c - 1, c + p));
            link(&mut ret, up(0, 2 * c + p), up(4 * c - 1, p));
            link(&mut ret, right(p, 3 * c - 1), left(2 * c + n, 2 * c - 1));
            link(&mut ret, right(c + p, 2 * c - 1), up(c - 1, 2 * c + p));
        }
    } else {
        panic!("Illegal file")
    }
    ret
}

#[derive(Clone)]
struct Solver {
    map: Map,
    position: Position,
    path: Vec<Step>,
    cube_links: HashMap<Position, Position>,
}

impl Solver {
    fn parse(file: &str) -> Self {
        let blocks = util::read_blocks(file);
        let map = Map::parse(&blocks[0]);
        let position = Position {
            row: 0,
            col: map.first_of_row(0),
            direction: Direction::Right,
        };
        let mut path = Vec::new();
        let mut number_str = "".to_string();
        for c in blocks[1][0].chars() {
            match c {
                'R' | 'L' => {
                    if !number_str.is_empty() {
                        for _ in 0..number_str.parse().unwrap() {
                            path.push(Step::Forward);
                        }
                        number_str.clear();
                        path.push(match c {
                            'R' => Step::Right,
                            'L' => Step::Left,
                            _ => panic!(),
                        })
                    }
                }
                _ => number_str.push(c),
            }
        }
        if !number_str.is_empty() {
            for _ in 0..number_str.parse().unwrap() {
                path.push(Step::Forward);
            }
        }
        let cube_links = calc_cube_links(file);
        Self {
            map,
            position,
            path,
            cube_links,
        }
    }

    #[allow(dead_code)]
    fn print_cube_links(&self) {
        let c = 50;
        let dir = Direction::Right;
        let min_row = 3 * c;
        let max_row = min_row + c;
        let mut letter = b'a';
        let mut link_map: HashMap<(usize, usize), u8> = HashMap::new();
        for (from, to) in &self.cube_links {
            if from.direction == dir && from.row >= min_row && from.row < max_row {
                link_map.insert((from.row, from.col), letter);
                link_map.insert((to.row, to.col), letter);
                if letter == b'z' {
                    letter = b'a'
                } else {
                    letter += 1;
                }
            }
        }
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Some(letter) = link_map.get(&(y, x)) {
                    print!("{}", *letter as char);
                } else if matches!(tile, Tile::Wrap) {
                    print!(" ")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }

    fn go_step(&mut self, step: &Step, on_cube: bool) {
        // println!("Step {:?} from {:#?} ", step, self.position);
        match step {
            Step::Left => self.position.direction.rotate_left(),
            Step::Right => self.position.direction.rotate_right(),
            Step::Forward => {
                if on_cube {
                    if let Some(position) = self.cube_links.get(&self.position) {
                        if !matches!(self.map.tiles[position.row][position.col], Tile::Block) {
                            self.position = *position;
                        }
                        return;
                    }
                }
                let mut row = self.position.row;
                let mut col = self.position.col;
                match self.position.direction {
                    Direction::Up => (row, col) = self.map.up_of(row, col),
                    Direction::Down => (row, col) = self.map.down_of(row, col),
                    Direction::Left => (row, col) = self.map.left_of(row, col),
                    Direction::Right => (row, col) = self.map.right_of(row, col),
                }
                match self.map.tiles[row][col] {
                    Tile::Empty => {
                        self.position.row = row;
                        self.position.col = col;
                    }
                    Tile::Block => (),
                    Tile::Wrap => panic!("Shouldn't land on wrap"),
                }
            }
        }
    }

    fn run(&mut self, on_cube: bool) {
        let mut fw = 0;
        for (i, step) in self.path.clone().iter().enumerate() {
            if i < 10000 {
                if let Step::Forward = step {
                    fw += 1;
                } else {
                    if fw > 0 {
                        println!("Forward {}", fw);
                        fw = 0;
                    }
                    self.print_state();
                    println!("\nStep {:?}", step);
                }
            }

            self.go_step(step, on_cube);
            // println!("{:?}", self.position);
        }
    }

    fn solution_pos(&self) -> (usize, usize) {
        (self.position.row + 1, self.position.col + 1)
    }

    fn print_state(&self) {
        println!();
        for (y, row) in self.map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let c = if self.position.row == y && self.position.col == x {
                    match self.position.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                } else {
                    match tile {
                        Tile::Block => '#',
                        Tile::Empty => '.',
                        Tile::Wrap => ' ',
                    }
                };
                print!("{}", c);
            }
            println!()
        }
    }
}

#[allow(dead_code)]
fn part_1(mut solver: Solver) {
    solver.run(false);
    let facing = solver.position.direction.to_facing();
    let (row, col) = solver.solution_pos();
    let res = 1000 * row + 4 * col + facing;
    println!("Part 1: {}", res);
}

fn part_2(mut solver: Solver) {
    solver.run(true);
    let facing = solver.position.direction.to_facing();
    let (row, col) = solver.solution_pos();
    let res = 1000 * row + 4 * col + facing;
    println!("Part 2: {}", res);
}

pub fn run() {
    // let solver = Solver::parse("22-test");
    let solver = Solver::parse("22-input");
    // solver.print_cube_links();
    // part_1(solver.clone());
    part_2(solver.clone());
}
