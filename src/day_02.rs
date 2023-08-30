use crate::util;

#[derive(Copy, Clone)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

use Shape::*;

fn int_to_shape(i: i32) -> Shape {
    match i {
        0 => Rock,
        1 => Paper,
        2 => Scissors,
        _ => panic!("Invalid shape"),
    }
}

#[derive(Copy, Clone, Debug)]
enum Res {
    Lose,
    Draw,
    Win,
}

use Res::*;

fn parse_opp_shape(opp_char: &str) -> Shape {
    match opp_char {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Invalid opponent shape"),
    }
}

fn parse_my_shape(my_char: &str) -> Shape {
    match my_char {
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Invalid opponent shape"),
    }
}

fn parse_result(result_char: &str) -> Res {
    match result_char {
        "X" => Lose,
        "Y" => Draw,
        "Z" => Win,
        _ => panic!("Invalid result"),
    }
}

fn score_1(opp_shape: Shape, my_shape: Shape) -> i32 {
    let shape_score = my_shape as i32 + 1;
    let diff = (3 + my_shape as i32 - opp_shape as i32) % 3;
    let win_score = match diff {
        0 => 3,
        1 => 6,
        _ => 0,
    };
    shape_score + win_score
}

fn score_2(opp_shape: Shape, result: Res) -> i32 {
    let my_shape: Shape = int_to_shape(match result {
        Lose => (opp_shape as i32 + 2) % 3,
        Draw => opp_shape as i32,
        Win => (opp_shape as i32 + 1) % 3,
    });
    let res = score_1(opp_shape, my_shape);
    res
}

fn parse_strategy_1(file: &String) -> Vec<(Shape, Shape)> {
    util::read_lines(file)
        .iter()
        .map(|line| {
            let tokens = util::tokenize(line);
            let opp_shape = parse_opp_shape(&tokens[0]);
            let my_shape = parse_my_shape(&tokens[1]);
            (opp_shape, my_shape)
        })
        .collect()
}

fn parse_strategy_2(file: &String) -> Vec<(Shape, Res)> {
    util::read_lines(file)
        .iter()
        .map(|line| {
            let tokens = util::tokenize(line);
            let opp_shape = parse_opp_shape(&tokens[0]);
            let res = parse_result(&tokens[1]);
            (opp_shape, res)
        })
        .collect()
}

pub fn part_1(file: &String) -> i32 {
    parse_strategy_1(file)
        .iter()
        .map(|(opp, my)| score_1(*opp, *my))
        .sum()
}

pub fn part_2(file: &String) -> i32 {
    parse_strategy_2(file)
        .iter()
        .map(|(opp, res)| score_2(*opp, *res))
        .sum()
}
