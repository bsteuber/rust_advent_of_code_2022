type Num = f64;
type Id = usize;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mult,
    Div,
}

use std::collections::HashMap;

use crate::util;

impl Operator {
    fn parse(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            "*" => Operator::Mult,
            "/" => Operator::Div,
            _ => panic!("Illegal operator {}", s),
        }
    }

    fn apply(&self, x: Num, y: Num) -> Num {
        match self {
            Operator::Add => x + y,
            Operator::Sub => x - y,
            Operator::Mult => x * y,
            Operator::Div => x / y,
        }
    }
}

#[derive(Clone, Debug)]
enum MonkeyJob {
    Number(Num),
    Operation(Id, Operator, Id),
}

#[derive(Clone)]
struct Solver {
    id_map: HashMap<String, Id>,
    next_id: Id,
    jobs: HashMap<Id, MonkeyJob>,
}

impl Solver {
    fn parse(file: &str) -> Self {
        let mut solver = Self {
            id_map: HashMap::new(),
            next_id: 0,
            jobs: HashMap::new(),
        };
        for line in util::read_lines(file) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let monkey = solver.lookup_monkey(&tokens[0].replace(":", ""));
            let job = match tokens[1].parse() {
                Ok(num) => MonkeyJob::Number(num),
                _ => {
                    let left = solver.lookup_monkey(tokens[1]);
                    let op = Operator::parse(tokens[2]);
                    let right = solver.lookup_monkey(tokens[3]);
                    MonkeyJob::Operation(left, op, right)
                }
            };
            solver.jobs.insert(monkey, job);
        }
        solver
    }
    fn lookup_monkey(&mut self, monkey: &str) -> Id {
        match self.id_map.get(monkey) {
            Some(id) => *id,
            None => {
                let id = self.next_id;
                self.id_map.insert(monkey.to_string(), id);
                self.next_id += 1;
                id
            }
        }
    }
    fn solve_for(&mut self, monkey: Id) -> Num {
        match self.jobs.get(&monkey).expect("Monkey not found") {
            MonkeyJob::Number(x) => *x,
            MonkeyJob::Operation(left, op, right) => {
                let left = *left;
                let right = *right;
                let op = *op;
                let x = self.solve_for(left);
                let y = self.solve_for(right);
                let res = op.apply(x, y);
                *self.jobs.get_mut(&monkey).unwrap() = MonkeyJob::Number(res);
                res
            }
        }
    }

    fn solve_part_1(&self) -> Num {
        let mut solver = self.clone();
        let root = solver.lookup_monkey("root");
        solver.solve_for(root)
    }

    fn try_value(&self, value: Num) -> Num {
        let mut solver = self.clone();
        let root = solver.lookup_monkey("root");
        let humn = solver.lookup_monkey("humn");
        match solver.jobs.get_mut(&root).unwrap() {
            MonkeyJob::Operation(_, op, _) => *op = Operator::Sub,
            _ => panic!("root has no operation????"),
        };
        *solver.jobs.get_mut(&humn).unwrap() = MonkeyJob::Number(value);
        // println!("Root: {}\nhumn: {}\nJobs:\n{:#?}", root, humn, solver.jobs);
        let res = solver.solve_for(root);
        // println!("Jobs after:\n{:#?}", solver.jobs);
        res
    }
}

fn part_2(solver: &Solver) {
    let mut start = 3000000000000u64; // > 0
    let mut end = 4000000000000u64; // < 0
    while start + 1 < end {
        let mid = (start + end) / 2;
        let res = solver.try_value(mid as f64);
        println!("Try {} -> {}", mid, res);
        if res > 0f64 {
            start = mid
        } else if res < 0f64 {
            end = mid
        } else {
            println!("Solved it! {}", mid);
            return;
        }
    }
    println!(
        "Didn't find it :(\nStart: {} -> {}, End: {} -> {}",
        start,
        solver.try_value(start as f64),
        end,
        solver.try_value(end as f64)
    );
}

pub fn run() {
    // let file = "21-test";
    let file = "21-input";
    let solver = Solver::parse(file);
    println!("Part 1: {}", solver.solve_part_1());
    println!("Part 2:");
    part_2(&solver);
}
