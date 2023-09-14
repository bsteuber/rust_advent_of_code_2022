use std::str::Chars;

use crate::util;

#[derive(Debug)]
enum Expr {
    Number(i64),
    List(Vec<Expr>),
}

use Expr::*;

impl Expr {
    fn compare(&self, other: &Self) -> i32 {
        // println!("Compare:\n  {:?}\n  {:?}", self, other);
        match (self, other) {
            (Number(x), Number(y)) => {
                if x < y {
                    -1
                } else if x == y {
                    0
                } else {
                    1
                }
            }
            (List(xs), List(ys)) => {
                let mut i = 0;
                loop {
                    // println!("Loop {}", i);
                    let left = xs.get(i);
                    let right = ys.get(i);
                    match (left, right) {
                        (None, None) => return 0,
                        (None, Some(_)) => return -1,
                        (Some(_), None) => return 1,
                        (Some(x), Some(y)) => {
                            let res = x.compare(y);
                            if res == 0 {
                                i += 1;
                                continue;
                            } else {
                                return res;
                            }
                        }
                    }
                }
            }
            (Number(x), List(_)) => {
                let left = List(vec![(Number(*x))]);
                left.compare(other)
            }
            (List(_), Number(y)) => {
                let right = List(vec![(Number(*y))]);
                self.compare(&right)
            }
        }
    }
}

struct Parser<'a> {
    current_char: Option<char>,
    iterator: Chars<'a>,
}

impl<'a> Parser<'a> {
    fn new(line: &'a str) -> Self {
        let mut iterator = line.chars();
        let current = iterator.next();
        Self {
            current_char: current,
            iterator,
        }
    }

    fn current(&self) -> char {
        self.current_char.expect("Unexpected end of line")
    }

    fn next(&mut self) {
        self.current_char = self.iterator.next();
    }

    fn parse_number(&mut self) -> i64 {
        let mut num_str = String::new();
        while matches!(self.current(), '0'..='9') {
            num_str.push(self.current());
            self.next();
        }
        num_str
            .parse()
            .expect(&format!("Failed to parse: {}", num_str))
    }

    fn parse_expr(&mut self) -> Expr {
        if self.current() == '[' {
            Expr::List(self.parse_list())
        } else {
            Expr::Number(self.parse_number())
        }
    }
    fn parse_list(&mut self) -> Vec<Expr> {
        let mut res = Vec::new();
        self.next();
        while self.current() != ']' {
            res.push(self.parse_expr());
            if self.current() == ',' {
                self.next()
            }
        }
        self.next();
        res
    }

    fn parse(line: &'a str) -> Expr {
        // println!("Parse line: {}", line);
        let mut parser = Self::new(line);
        let res = parser.parse_expr();
        assert!(
            parser.iterator.next().is_none(),
            "Expression ended before end of line"
        );
        res
    }
}

#[derive(Debug)]
struct PacketPair {
    left: Expr,
    right: Expr,
}

impl PacketPair {
    fn parse(block: &Vec<String>) -> Self {
        Self {
            left: Parser::parse(&block[0]),
            right: Parser::parse(&block[1]),
        }
    }

    fn is_in_order(&self) -> bool {
        self.left.compare(&self.right) == -1
    }
}

pub fn part_1(file: &str) -> usize {
    let packet_pairs: Vec<PacketPair> = util::read_blocks(file)
        .iter()
        .map(PacketPair::parse)
        .collect();
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.is_in_order())
        .map(|(i, _)| i + 1)
        .sum()
}

fn insert_sorted<'a>(v: &mut Vec<&'a Expr>, e: &'a Expr) {
    let search = v.iter().enumerate().find(|(_, item)| e.compare(item) == -1);
    match search {
        None => v.push(e),
        Some((index, _)) => v.insert(index, e),
    }
}

pub fn part_2(file: &str) -> usize {
    let packet_pairs: Vec<PacketPair> = util::read_blocks(file)
        .iter()
        .map(PacketPair::parse)
        .collect();
    let mut v: Vec<&Expr> = Vec::new();
    let diviver_1 = Parser::parse("[[2]]");
    let diviver_2 = Parser::parse("[[6]]");
    {
        insert_sorted(&mut v, &diviver_1);
    }
    {
        insert_sorted(&mut v, &diviver_2);
    }
    for pair in &packet_pairs {
        {
            insert_sorted(&mut v, &pair.left);
        }
        {
            insert_sorted(&mut v, &pair.right);
        }
    }
    fn find_packet(v: &Vec<&Expr>, e: &Expr) -> Option<usize> {
        let r = v.iter().enumerate().find(|(_, item)| e.compare(item) == 0);
        r.map(|t| t.0 + 1)
    }
    let p1 = find_packet(&v, &diviver_1).expect("Didn't find divider 1");
    let p2 = find_packet(&v, &diviver_2).expect("Didn't find divider 2");
    p1 * p2
}
