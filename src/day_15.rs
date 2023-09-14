use std::collections::HashSet;

use crate::util;

type Num = i32;

#[derive(Debug)]
struct Point {
    x: Num,
    y: Num,
}

impl Point {
    fn distance(&self, other: &Point) -> Num {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let d = dx + dy;
        // println!(
        //     "Distance from {} to {}: dx -> {}, dy -> {}, d -> {}",
        //     self, other, dx, dy, d
        // );
        d
    }
}

#[derive(Debug)]
struct SensorData {
    sensor: Point,
    closest_beacon: Point,
    radius: Num,
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    first: Num,
    last: Num,
}

impl SensorData {
    fn parse(line: &str) -> Self {
        let line = line
            .replace(",", "")
            .replace(":", "")
            .replace("x=", "")
            .replace("y=", "");
        let tokens: Vec<&str> = line.split(" ").collect();
        let sensor = Point {
            x: tokens[2].parse().unwrap(),
            y: tokens[3].parse().unwrap(),
        };
        let closest_beacon = Point {
            x: tokens[8].parse().unwrap(),
            y: tokens[9].parse().unwrap(),
        };
        let radius = (&sensor).distance(&closest_beacon);
        Self {
            sensor,
            closest_beacon,
            radius,
        }
    }

    fn cover_on(&self, line: Num) -> Option<Interval> {
        let diff_y = (self.sensor.y - line).abs();
        let radius_on_line = self.radius - diff_y;
        if radius_on_line >= 0 {
            Some(Interval {
                first: self.sensor.x - radius_on_line,
                last: self.sensor.x + radius_on_line,
            })
        } else {
            None
        }
    }
}

struct SensorMap {
    data: Vec<SensorData>,
}

impl SensorMap {
    fn parse(file: &str) -> Self {
        let data = util::read_lines(file)
            .iter()
            .map(|line| SensorData::parse(line))
            .collect();
        Self { data }
    }

    fn covered_count(&self, line: Num) -> usize {
        let mut covered = HashSet::new();
        let beacons_on_line: HashSet<Num> = self
            .data
            .iter()
            .filter(|sensor_data| sensor_data.closest_beacon.y == line)
            .map(|sensor_data| sensor_data.closest_beacon.x)
            .collect();
        for sensor_data in &self.data {
            if let Some(interval) = sensor_data.cover_on(line) {
                println!(
                    "Cover of sensor {:?} with radius {} in line {} is {:?}",
                    sensor_data.sensor, sensor_data.radius, line, interval
                );
                for x in interval.first..interval.last + 1 {
                    if !beacons_on_line.get(&x).is_some() {
                        covered.insert(x);
                    }
                }
            }
        }
        covered.len()
    }

    fn find_only_sender(&self, max_xy: Num) -> Point {
        let mut only_result: Option<Point> = None;
        'lines: for y in 0..max_xy + 1 {
            // println!("\n\n y = {}", y);
            let beacons_on_line: HashSet<Num> = self
                .data
                .iter()
                .filter(|sensor_data| sensor_data.closest_beacon.y == y)
                .map(|sensor_data| sensor_data.closest_beacon.x)
                .collect();
            let mut intervals = Vec::new();
            for sensor_data in &self.data {
                if let Some(interval) = sensor_data.cover_on(y) {
                    // println!("  interval: {:?}", interval);
                    intervals.push(interval);
                }
            }
            intervals.sort_unstable_by(|i1, i2| i1.first.cmp(&i2.first));
            let mut next_possible: Num = 0;
            for Interval { first, last } in intervals.iter() {
                if next_possible > max_xy {
                    continue 'lines;
                }
                for x in next_possible..*first {
                    // print!("  x = {}", x);
                    if beacons_on_line.get(&x).is_none() {
                        if only_result.is_some() {
                            panic!("More than one possible position!")
                        } else {
                            only_result = Some(Point { x, y })
                        }
                    }
                }
                next_possible = next_possible.max(last + 1)
            }
        }
        only_result.expect("No solution found")
    }
}

pub fn part_1(file: &str, line: Num) -> usize {
    let map = SensorMap::parse(file);
    map.covered_count(line)
}

pub fn part_2(file: &str, max_xy: Num) -> u64 {
    let map = SensorMap::parse(file);
    let Point { x, y } = map.find_only_sender(max_xy);
    (x as u64) * 4000000 + (y as u64)
}
