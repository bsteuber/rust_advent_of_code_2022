mod day_04;
use day_04::*;
mod util;

const DAY: &str = "04";

#[allow(unused_variables)]
fn main() {
    let test_file = format!("{}-test", DAY);
    let input_file = format!("{}-input", DAY);
    println!("Part 1 Test: {}", part_1(&test_file));
    println!("Part 1 Result: {}", part_1(&input_file));
    println!("Part 2 Test: {}", part_2(&test_file));
    println!("Part 2 Result: {}", part_2(&input_file));
}
