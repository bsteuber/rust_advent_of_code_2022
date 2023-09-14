use crate::util;

#[derive(Debug)]
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

struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

struct Solver {
    map: Map,
    position: Position,
    path: Vec<Step>,
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
        Self {
            map,
            position,
            path,
        }
    }

    fn go_step(&mut self, step: &Step) {
        // println!("Step {:?} from {:#?} ", step, self.position);
        match step {
            Step::Left => self.position.direction.rotate_left(),
            Step::Right => self.position.direction.rotate_right(),
            Step::Forward => {
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

    fn run(&mut self) {
        for step in &self.path.clone() {
            println!("\nStep {:?}", step);
            self.go_step(step);
            println!("{:?}", self.position);
        }
    }

    fn solution_pos(&self) -> (usize, usize) {
        (self.position.row + 1, self.position.col + 1)
    }
}

pub fn run() {
    // let mut solver = Solver::parse("22-test");
    let mut solver = Solver::parse("22-input");
    // println!("First row: {:?}", solver.map.tiles[0]);
    // println!("Initial pos: {:?}", solver.position);
    solver.run();
    let facing = solver.position.direction.to_facing();
    let (row, col) = solver.solution_pos();
    let res = 1000 * row + 4 * col + facing;
    println!("Part 1: {}", res)
}
