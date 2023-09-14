use std::collections::HashSet;

type Num = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: Num,
    y: Num,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    DownLeft,
    DownRight,
}

use Dir::*;

use crate::util;

impl Point {
    fn step(self, dir: Dir) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        match dir {
            Up => y -= 1,
            Down => y += 1,
            Left => x -= 1,
            Right => x += 1,
            DownLeft => {
                x -= 1;
                y += 1;
            }
            DownRight => {
                x += 1;
                y += 1;
            }
        }
        Self { x, y }
    }

    fn dir_of(&self, other: &Self) -> Dir {
        match ((other.x - self.x).signum(), (other.y - self.y).signum()) {
            (0, -1) => Up,
            (0, 1) => Down,
            (-1, 0) => Left,
            (1, 0) => Right,
            _ => panic!("other point not vertical or horizontal from this one"),
        }
    }
}

struct Path {
    points: Vec<Point>,
}

impl Path {
    fn parse(line: &str) -> Self {
        Self {
            points: line
                .split(" -> ")
                .map(|point_str| {
                    let mut iter = point_str.split(",");
                    let x = iter.next().unwrap().parse().unwrap();
                    let y = iter.next().unwrap().parse().unwrap();
                    Point { x, y }
                })
                .collect(),
        }
    }

    fn points(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        let mut points = self.points.iter();
        let mut current = *points.next().unwrap();
        ret.push(current);
        while let Some(end) = points.next() {
            let dir = current.dir_of(end);
            while current != *end {
                // println!("cur {:?}, end: {:?}", current, end);
                current = current.step(dir);
                // println!("after step in {:?} => {:?}", dir, current);
                ret.push(current);
            }
        }
        ret
    }
}

struct Grid {
    fields: HashSet<Point>,
    max_y: Num,
    walled: bool,
}

impl Grid {
    fn parse(file: &str, walled: bool) -> Self {
        let mut fields = HashSet::new();
        let mut lowest = 0;
        for line in util::read_lines(file) {
            // println!("Line: {}", line);
            let path = Path::parse(&line);
            for point in path.points() {
                // println!("Pt {:?}", point);
                if point.y > lowest {
                    lowest = point.y;
                }
                fields.insert(point);
            }
        }
        Self {
            fields,
            max_y: lowest,
            walled,
        }
    }

    fn is_occupied(&self, point: &Point) -> bool {
        self.fields.get(point).is_some() || (self.walled && point.y == self.max_y + 2)
    }

    fn simulate_one_sand(&mut self) -> bool {
        let mut current = Point { x: 500, y: 0 };
        while current.y < self.max_y + 3 {
            // println!("Current: {:?}, Max_y: {}", current, self.max_y);
            let next = current.step(Down);
            if !self.is_occupied(&next) {
                current = next;
                continue;
            }
            let next = current.step(DownLeft);
            if !self.is_occupied(&next) {
                current = next;
                continue;
            }
            let next = current.step(DownRight);
            if !self.is_occupied(&next) {
                current = next;
                continue;
            }
            return self.fields.insert(current);
        }
        false
    }

    fn simulate_all(&mut self) -> usize {
        let mut count = 0;
        while self.simulate_one_sand() {
            // println!("Simulated {}", count);
            count += 1;
        }
        count
    }
}

pub fn part_1(file: &str) -> usize {
    let mut grid = Grid::parse(file, false);
    // println!("parsed");
    grid.simulate_all()
}

pub fn part_2(file: &str) -> usize {
    let mut grid = Grid::parse(file, true);
    // println!("parsed");
    grid.simulate_all()
}
