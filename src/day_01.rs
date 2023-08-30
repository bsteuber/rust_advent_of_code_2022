use crate::util;

fn elf_calories(file: &String) -> Vec<i32> {
    return util::read_int_blocks(file)
        .iter()
        .map(|block| block.iter().sum::<i32>())
        .collect();
}

pub fn part_1(file: &String) -> i32 {
    return *elf_calories(file).iter().max().unwrap();
}

pub fn part_2(file: &String) -> i32 {
    let mut cals = elf_calories(file);
    cals.sort();
    cals.reverse();
    return cals.iter().take(3).cloned().sum();
}
