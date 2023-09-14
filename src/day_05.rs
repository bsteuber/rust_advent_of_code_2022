use crate::util;

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn parse(lines: &Vec<String>) -> Self {
        let mut stacks = Vec::new();

        let boxes: Vec<String> = lines
            .iter()
            .rev()
            .skip(1)
            .map(|line|
        // iterate over all chars in line whose position % 4 == 1
        line.chars().enumerate().filter(|(i, _)| i % 4 == 1).map(|(_, c)| c).collect())
            .collect();

        for row in boxes {
            for (i, c) in row.chars().enumerate() {
                if c != ' ' {
                    if stacks.len() == i {
                        stacks.push(Vec::new());
                    }
                    stacks[i].push(c);
                }
            }
        }
        Self { stacks }
    }

    fn apply_step_1(&mut self, step: &Step) {
        for _ in 0..step.amount {
            let item = self.stacks[step.from].pop().unwrap();
            self.stacks[step.to].push(item)
        }
    }

    fn apply_step_2(&mut self, step: &Step) {
        let mut tmp = Vec::new();
        for _ in 0..step.amount {
            tmp.push(self.stacks[step.from].pop().unwrap());
        }
        for item in tmp.iter().rev() {
            self.stacks[step.to].push(*item)
        }
    }

    fn result(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}

#[derive(Debug)]
struct Step {
    amount: i32,
    from: usize,
    to: usize,
}
impl Step {
    fn parse(line: &str) -> Self {
        let tokens = util::tokenize(&line);
        let amount = tokens[1].parse().unwrap();
        let from = (tokens[3].parse::<i32>().unwrap() - 1) as usize;
        let to = (tokens[5].parse::<i32>().unwrap() - 1) as usize;
        Self { amount, from, to }
    }
}

type Steps = Vec<Step>;

#[derive(Debug)]
struct Input {
    state: State,
    steps: Steps,
}

impl Input {
    fn parse(file: &str) -> Self {
        let blocks = util::read_blocks(file);
        let state_lines = &blocks[0];
        let step_lines = &blocks[1];

        let mut steps = Vec::new();
        for line in step_lines {
            steps.push(Step::parse(line))
        }
        Input {
            state: State::parse(state_lines),
            steps: steps,
        }
    }

    fn apply_all_1(&mut self) {
        for step in &self.steps {
            self.state.apply_step_1(step)
        }
    }

    fn apply_all_2(&mut self) {
        for step in &self.steps {
            self.state.apply_step_2(step)
        }
    }
}

pub fn part_1(file: &str) -> String {
    let mut input = Input::parse(file);
    input.apply_all_1();
    input.state.result()
}

pub fn part_2(file: &str) -> String {
    let mut input = Input::parse(file);
    input.apply_all_2();
    input.state.result()
}
