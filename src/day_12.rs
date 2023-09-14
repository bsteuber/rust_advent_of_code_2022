use std::collections::{HashMap, HashSet};

use crate::util;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug)]
struct HeightMap {
    field: Vec<Vec<u8>>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn parse(lines: &[String]) -> Self {
        let map_height = lines.len();
        let map_width = lines[0].bytes().len();
        let mut start = Point(0, 0);
        let mut end = Point(0, 1);
        let mut field: Vec<Vec<u8>> = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            let mut row: Vec<u8> = Vec::new();
            for (x, char) in line.bytes().enumerate() {
                let height = match char {
                    b'S' => {
                        start = Point(x, y);
                        b'a'
                    }
                    b'E' => {
                        end = Point(x, y);
                        b'z'
                    }
                    _ => char,
                };
                row.push(height);
            }
            field.push(row);
        }
        Self {
            field,
            start,
            end,
            width: map_width,
            height: map_height,
        }
    }

    fn point_height(&self, p: &Point) -> u8 {
        self.field[p.1][p.0]
    }

    fn neighbours(&self, Point(x, y): &Point) -> Vec<Point> {
        let mut res = Vec::new();
        if *x > 0usize {
            res.push(Point(x - 1, *y));
        }
        if *y > 0usize {
            res.push(Point(*x, y - 1));
        }
        if x + 1 < self.width {
            res.push(Point(x + 1, *y));
        }
        if y + 1 < self.height {
            res.push(Point(*x, y + 1));
        }
        res
    }

    fn shortest_path(&self, start: &[Point]) -> usize {
        let mut steps = 0;
        let mut steps_to: HashMap<Point, usize> = HashMap::new();
        let mut search_now: HashSet<Point> = HashSet::new();
        for start_point in start {
            steps_to.insert(*start_point, 0);
            search_now.insert(*start_point);
        }

        loop {
            // println!("{:?}", steps_to);
            steps += 1;
            let mut search_next = HashSet::new();
            for point in &search_now {
                let max_neighb_height = self.point_height(point) + 1;
                for neighbor in self.neighbours(point) {
                    if self.point_height(&neighbor) <= max_neighb_height {
                        if neighbor == self.end {
                            return steps;
                        } else {
                            match steps_to.get(&neighbor) {
                                Some(_) => (),
                                None => {
                                    steps_to.insert(neighbor, steps);
                                    search_next.insert(neighbor);
                                }
                            }
                        }
                    }
                }
            }
            search_now = search_next;
        }
    }

    fn lowest_points(&self) -> Vec<Point> {
        let mut v = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let p = Point(x, y);
                if self.point_height(&p) == b'a' {
                    v.push(p);
                }
            }
        }
        v
    }
}

pub fn part_1(file: &str) -> usize {
    let map = HeightMap::parse(&util::read_lines(file));
    map.shortest_path(&vec![map.start])
}

pub fn part_2(file: &str) -> usize {
    let map = HeightMap::parse(&util::read_lines(file));
    map.shortest_path(&map.lowest_points())
}
