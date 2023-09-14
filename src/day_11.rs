use std::collections::VecDeque;

use crate::util;

#[derive(Debug)]
enum Operator {
    Plus,
    Times,
}

impl Operator {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            Self::Plus => left + right,
            Self::Times => left * right,
        }
    }

    fn parse(s: &str) -> Self {
        match s {
            "+" => Self::Plus,
            "*" => Self::Times,
            _ => panic!("Illegal operator: {}", s),
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Number(usize),
}

impl Operand {
    fn value(&self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Number(x) => *x,
        }
    }

    fn parse(s: &str) -> Self {
        if s == "old" {
            Operand::Old
        } else {
            Operand::Number(s.parse().expect("Operand is not a number!"))
        }
    }
}

#[derive(Debug)]
struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Operation {
    fn apply(&self, old: usize) -> usize {
        self.operator
            .apply(self.left.value(old), self.right.value(old))
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test_divisor: usize,
    test_true_monkey: usize,
    test_false_monkey: usize,
    activeness: usize,
}

impl Monkey {
    fn parse<'a>(lines: &Vec<String>) -> Self {
        let mut monkey = Self {
            items: VecDeque::new(),
            operation: Operation {
                left: Operand::Old,
                operator: Operator::Plus,
                right: Operand::Old,
            },
            test_divisor: 0,
            test_true_monkey: 0,
            test_false_monkey: 0,
            activeness: 0,
        };
        for line in lines {
            let line = line.replace(",", "");
            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens[0] {
                "Monkey" => (),
                "Starting" => {
                    for item in tokens[2..].iter() {
                        monkey.items.push_back(item.parse().unwrap());
                    }
                }
                "Operation:" => {
                    monkey.operation.left = Operand::parse(tokens[3]);
                    monkey.operation.operator = Operator::parse(tokens[4]);
                    monkey.operation.right = Operand::parse(tokens[5])
                }
                "Test:" => monkey.test_divisor = tokens[3].parse().unwrap(),
                "If" if tokens[1] == "true:" => {
                    monkey.test_true_monkey = tokens[5].parse().unwrap()
                }
                "If" if tokens[1] == "false:" => {
                    monkey.test_false_monkey = tokens[5].parse().unwrap()
                }
                _ => panic!("Can't parse {}", tokens[0]),
            }
        }
        monkey
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    mod_divisor: usize,
}

impl Monkeys {
    fn parse(blocks: Vec<Vec<String>>) -> Self {
        let monkeys: Vec<Monkey> = blocks.iter().map(Monkey::parse).collect();
        let mod_divisor = monkeys.iter().map(|monkey| monkey.test_divisor).product();
        Self {
            monkeys,
            mod_divisor,
        }
    }

    fn make_turn(&mut self, monkey_id: usize, do_relief: bool) {
        while let Some(item) = self.monkeys[monkey_id].items.pop_front() {
            let monkey = &mut self.monkeys[monkey_id];
            monkey.activeness += 1;
            let Monkey {
                operation,
                test_divisor,
                test_false_monkey,
                test_true_monkey,
                items: _,
                activeness: _,
            } = monkey;
            // println!("inspecting {}", item);
            let item = operation.apply(item);
            // println!("worry level is now {}", item);
            let item = if do_relief {
                item / 3
            } else {
                item % self.mod_divisor
            };
            // println!("Gets divided to {}", item);
            let next_monkey = if item % *test_divisor == 0 {
                *test_true_monkey
            } else {
                *test_false_monkey
            };
            // println!("Next monkey is {}", next_monkey);
            self.monkeys[next_monkey].items.push_back(item);
            // panic!("Stop");
        }
    }

    fn run_round(&mut self, do_relief: bool) {
        for monkey_id in 0..self.monkeys.len() {
            self.make_turn(monkey_id, do_relief);
        }
    }

    fn print_items(&self) {
        for (id, monkey) in self.monkeys.iter().enumerate() {
            println!(
                "Monkey {}: {}",
                id,
                monkey
                    .items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }
    }

    fn monkey_business(&self) -> usize {
        let mut activenesses: Vec<usize> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.activeness)
            .collect();

        activenesses.sort();
        if let &[x, y] = &activenesses[activenesses.len() - 2..] {
            x * y
        } else {
            panic!("Need at least two monkeys");
        }
    }
}

pub fn part_1(file: &str) -> usize {
    let mut monkeys = Monkeys::parse(util::read_blocks(file));
    for _ in 0..20 {
        monkeys.run_round(true);
    }

    // monkeys.print_items();
    monkeys.monkey_business()
}

pub fn part_2(file: &str) -> usize {
    let mut monkeys = Monkeys::parse(util::read_blocks(file));
    for _ in 0..10000 {
        monkeys.run_round(false);
    }

    // monkeys.print_items();
    monkeys.monkey_business()
}
