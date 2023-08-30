use crate::util;
use std::collections::HashSet;

fn find_duplicate(line: &String) -> char {
    let half_len = line.len() >> 1;
    let first_half = &line[..half_len];
    let second_half = &line[half_len..];
    let first_item_set = first_half.chars().collect::<HashSet<char>>();
    let second_item_set = second_half.chars().collect::<HashSet<char>>();
    *first_item_set
        .intersection(&second_item_set)
        .next()
        .unwrap()
}
fn char_prio(c: char) -> i32 {
    let char_code = c as u8;
    match char_code {
        b'a'..=b'z' => 1 + ((char_code - b'a') as i32),
        b'A'..=b'Z' => 27 + ((char_code - b'A') as i32),
        _ => panic!("Invalid char: {}", c),
    }
}

pub fn part_1(file: &String) -> i32 {
    util::read_lines(file)
        .iter()
        .map(find_duplicate)
        .map(char_prio)
        .sum()
}

fn find_shared_item(line_1: &String, line_2: &String, line_3: &String) -> char {
    let line_1_set = line_1.chars().collect::<HashSet<char>>();
    let line_2_set = line_2.chars().collect::<HashSet<char>>();
    let line_3_set = line_3.chars().collect::<HashSet<char>>();
    *line_1_set
        .intersection(&line_2_set)
        .find(|c| line_3_set.contains(c))
        .unwrap()
}

pub fn part_2(file: &String) -> i32 {
    let lines = util::read_lines(file);
    let mut i = 0;
    let mut sum = 0;
    while i < lines.len() {
        let line_1 = &lines[i];
        let line_2 = &lines[i + 1];
        let line_3 = &lines[i + 2];
        let shared_item = find_shared_item(line_1, line_2, line_3);
        sum += char_prio(shared_item);
        i += 3;
    }
    sum
}
