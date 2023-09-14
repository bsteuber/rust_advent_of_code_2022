use std::collections::HashSet;

use crate::util;

type Coord = i32;
#[derive(Clone, Copy, Debug, PartialOrd, Ord, Hash, PartialEq, Eq)]
struct Point3D {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Point3D {
    fn parse(line: &String) -> Self {
        let tokens: Vec<Coord> = line.split(",").map(|x| x.parse().unwrap()).collect();
        Self {
            x: tokens[0],
            y: tokens[1],
            z: tokens[2],
        }
    }

    fn neighbours(&self) -> Vec<Self> {
        let Point3D { x, y, z } = self;
        vec![
            Point3D {
                x: x - 1,
                y: *y,
                z: *z,
            },
            Point3D {
                x: x + 1,
                y: *y,
                z: *z,
            },
            Point3D {
                x: *x,
                y: y - 1,
                z: *z,
            },
            Point3D {
                x: *x,
                y: y + 1,
                z: *z,
            },
            Point3D {
                x: *x,
                y: *y,
                z: z - 1,
            },
            Point3D {
                x: *x,
                y: *y,
                z: z + 1,
            },
        ]
    }
}

struct LavaMap {
    blocks: HashSet<Point3D>,
    min_x: Coord,
    max_x: Coord,
    min_y: Coord,
    max_y: Coord,
    min_z: Coord,
    max_z: Coord,
}

impl LavaMap {
    fn parse(file: &str) -> Self {
        let blocks: HashSet<Point3D> = util::read_lines(file).iter().map(Point3D::parse).collect();
        let min_x = blocks.iter().map(|p| p.x).min().unwrap();
        let max_x = blocks.iter().map(|p| p.x).max().unwrap();
        let min_y = blocks.iter().map(|p| p.y).min().unwrap();
        let max_y = blocks.iter().map(|p| p.y).max().unwrap();
        let min_z = blocks.iter().map(|p| p.z).min().unwrap();
        let max_z = blocks.iter().map(|p| p.z).max().unwrap();
        Self {
            blocks,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    fn list_surfaces(&self) -> Vec<(Point3D, Point3D)> {
        let mut res = Vec::new();
        for block in &self.blocks {
            for neighbour in block.neighbours() {
                if !self.blocks.contains(&neighbour) {
                    res.push((*block, neighbour));
                }
            }
        }
        res
    }

    fn count_surfaces(&self) -> usize {
        self.list_surfaces().len()
    }

    fn bounding_box(&self) -> HashSet<Point3D> {
        let mut res = HashSet::new();
        let x_range = self.min_x..self.max_x + 1;
        let y_range = self.min_y..self.max_y + 1;
        let z_range = self.min_z..self.max_z + 1;
        for x in x_range.clone() {
            for y in y_range.clone() {
                res.insert(Point3D {
                    x,
                    y,
                    z: self.min_z - 1,
                });
                res.insert(Point3D {
                    x,
                    y,
                    z: self.max_z + 1,
                });
            }
        }
        for x in x_range.clone() {
            for z in z_range.clone() {
                res.insert(Point3D {
                    x,
                    y: self.min_y - 1,
                    z,
                });
                res.insert(Point3D {
                    x,
                    y: self.max_y + 1,
                    z,
                });
            }
        }
        for z in z_range.clone() {
            for y in y_range.clone() {
                res.insert(Point3D {
                    x: self.min_x - 1,
                    y,
                    z,
                });
                res.insert(Point3D {
                    x: self.max_x + 1,
                    y,
                    z,
                });
            }
        }
        res
    }

    fn on_map(&self, p: &Point3D) -> bool {
        p.x >= self.min_x
            && p.y >= self.min_y
            && p.z >= self.min_z
            && p.x <= self.max_x
            && p.y <= self.max_y
            && p.z <= self.max_z
    }

    fn count_outer_surfaces(&self) -> usize {
        let mut checked: HashSet<Point3D> = HashSet::new();
        let mut surfaces = 0;
        let mut to_check = self.bounding_box();
        let mut check_next = HashSet::new();
        let simple_surfaces = self.list_surfaces();

        while !to_check.is_empty() {
            for point in &to_check {
                if checked.contains(point) {
                    continue;
                }
                // println!("Checking point {:?}", point);
                for neighbour in point.neighbours() {
                    // println!("Checking neighbour {:?}", neighbour);
                    if self.blocks.contains(&neighbour) {
                        // println!("Found {:?} -> {:?}", point, neighbour);
                        if simple_surfaces
                            .iter()
                            .find(|(block, empty)| empty == point && *block == neighbour)
                            .is_none()
                        {
                            panic!("Should't contain {:?} -> {:?}", point, neighbour);
                        }
                        surfaces += 1;
                    } else if !checked.contains(&neighbour) && self.on_map(&neighbour) {
                        check_next.insert(neighbour);
                    }
                }
                checked.insert(*point);
            }
            to_check = check_next;
            check_next = HashSet::new();
        }
        surfaces
    }
}

pub fn run() {
    // let map = LavaMap::parse("18-test");
    let map = LavaMap::parse("18-input");
    println!("Part 1: {}", map.count_surfaces());

    println!("Part 2: {}", map.count_outer_surfaces());
}
