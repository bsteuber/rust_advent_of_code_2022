use crate::util;

#[derive(Copy, Clone, Debug)]
struct DeviceState {
    cycle: i64,
    x: i64,
}

struct Device {
    state: DeviceState,
    history: Vec<DeviceState>,
}

impl Device {
    fn new() -> Self {
        let init_state = DeviceState { cycle: 1, x: 1 };
        Device {
            state: init_state,
            history: vec![],
        }
    }

    fn next_cycle(&mut self) {
        self.history.push(self.state);
        self.state.cycle += 1;
    }

    fn apply_command(&mut self, line: &str) {
        let mut iter = line.split_whitespace();
        let command = iter.next().unwrap();
        match command {
            "noop" => self.next_cycle(),
            "addx" => {
                let y: i64 = iter.next().unwrap().parse().unwrap();
                self.next_cycle();
                self.next_cycle();
                self.state.x += y;
            }
            _ => panic!("Invalid command"),
        }
    }

    fn apply_all<'a>(&mut self, lines: impl Iterator<Item = &'a str>) {
        for line in lines {
            self.apply_command(line);
        }
        self.next_cycle();
    }
}

pub fn part_1(file: &str) -> i64 {
    let mut device = Device::new();
    device.apply_all(util::read_lines(file).iter().map(|s| s.as_str()));
    device
        .history
        .iter()
        .filter(|state| (state.cycle + 20) % 40 == 0)
        .map(|state| state.cycle * state.x)
        .sum()
}

pub fn part_2(file: &str) -> String {
    let mut device = Device::new();
    device.apply_all(util::read_lines(file).iter().map(|s| s.as_str()));
    let mut out = String::new();
    for state in device.history {
        let cycle_pos = (state.cycle - 1) % 40;
        if cycle_pos == 0 {
            out.push_str("\n");
        }
        let pixel = if (cycle_pos - state.x).abs() <= 1 {
            "#"
        } else {
            "."
        };
        out.push_str(pixel);
    }
    out
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_device() {
        let mut device = Device::new();
        let lines = vec!["noop", "addx 3", "addx -5"];
        device.apply_all(lines.iter().copied());
        for state in &device.history {
            println!("{:?}", state);
        }
    }
}
