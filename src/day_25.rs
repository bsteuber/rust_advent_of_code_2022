use crate::util;

type Num = i64;

fn snafu_to_num(s: &str) -> Num {
    let mut res = 0;
    for c in s.chars() {
        res *= 5;
        res += match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Illegal snafu character"),
        }
    }
    res
}

fn num_to_snafu(x: Num) -> String {
    if x == 0 {
        return "0".to_string();
    }
    let mut reverse_res = String::new();
    let mut x = x;
    while x != 0 {
        let rest = ((x % 5) + 5) % 5;
        let (c, y) = match rest {
            0 => ('0', 0),
            1 => ('1', -1),
            2 => ('2', -2),
            3 => ('=', 2),
            4 => ('-', 1),
            _ => panic!("illegal modulo"),
        };
        reverse_res.push(c);
        x += y;
        x /= 5;
    }
    reverse_res.chars().rev().collect()
}

pub fn run() {
    let part_1 = util::read_lines("25-input")
        .iter()
        .map(|s| snafu_to_num(s))
        .sum();
    println!("Part 1: {}, {}", part_1, num_to_snafu(part_1));
}

#[cfg(test)]
mod test {

    use crate::util;

    use super::*;

    #[test]
    fn test_snafu_to_num() {
        for line in util::read_lines("25-samples") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let dezimal: Num = tokens[0].parse().unwrap();
            let snafu = snafu_to_num(tokens[1]);
            assert_eq!(dezimal, snafu)
        }
    }

    #[test]
    fn test_num_to_snafu() {
        for line in util::read_lines("25-samples") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let dezimal_str = num_to_snafu(tokens[0].parse().unwrap());
            let snafu_str = tokens[1];
            assert_eq!(dezimal_str, snafu_str)
        }
    }
}
