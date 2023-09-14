use crate::util;
use std::collections::HashSet;

fn detect_start_of_packet(s: &str, packet_len: usize) -> Option<usize> {
    for n in packet_len..=s.len() {
        let different_chars: HashSet<char> = s[n - packet_len..n].chars().collect();
        if different_chars.len() == packet_len {
            return Some(n);
        }
    }
    return None;
}

pub fn part_1(file: &str) -> usize {
    let lines = util::read_lines(file);
    let s = lines[0].trim();
    detect_start_of_packet(s, 4).unwrap()
}

pub fn part_2(file: &str) -> usize {
    let lines = util::read_lines(file);
    let s = lines[0].trim();
    detect_start_of_packet(s, 14).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_packet() {
        for (s, res) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ] {
            assert_eq!(detect_start_of_packet(s, 4), Some(res))
        }
    }

    #[test]
    fn test_start_of_message() {
        for (s, res) in [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ] {
            assert_eq!(detect_start_of_packet(s, 14), Some(res))
        }
    }
}
