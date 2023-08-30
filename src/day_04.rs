use crate::util;

type Assign = std::ops::RangeInclusive<i32>;

fn parse_assignment(assignment: &str) -> Assign {
    let tokens: Vec<i32> = assignment
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    tokens[0]..=tokens[1]
}

fn parse_assignments(line: &String) -> (Assign, Assign) {
    let tokens: Vec<&str> = line.split(",").collect();
    (parse_assignment(tokens[0]), parse_assignment(tokens[1]))
}

fn assign_contains(assign: &Assign, sub_assign: &Assign) -> bool {
    assign.start() <= sub_assign.start() && assign.end() <= sub_assign.end()
}

fn one_contains_another(assign_1: &Assign, assign_2: &Assign) -> bool {
    assign_contains(assign_1, assign_2) || assign_contains(assign_2, assign_1)
}

fn assignments_overlap(assign_1: &Assign, assign_2: &Assign) -> bool {
    assign_1.start() <= assign_2.end() && assign_2.start() <= assign_1.end()
}

pub fn part_1(file: &str) -> i32 {
    util::read_lines(file)
        .iter()
        .map(parse_assignments)
        .filter(|(assign_1, assign_2)| one_contains_another(assign_1, assign_2))
        .count() as i32
}

pub fn part_2(file: &str) -> i32 {
    util::read_lines(file)
        .iter()
        .map(parse_assignments)
        .filter(|(assign_1, assign_2)| assignments_overlap(assign_1, assign_2))
        .count() as i32
}
