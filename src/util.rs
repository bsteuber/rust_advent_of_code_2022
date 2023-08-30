#[allow(dead_code)]
pub fn read_lines(file: &str) -> Vec<String> {
    let path = format!("data/{}.txt", file);
    std::fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn lines_to_blocks(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut block = Vec::new();
    for line in lines {
        if line.is_empty() {
            if !block.is_empty() {
                blocks.push(block);
                block = Vec::new();
            }
        } else {
            block.push(line);
        }
    }
    if !block.is_empty() {
        blocks.push(block);
    }

    blocks
}

#[allow(dead_code)]
pub fn tokenize(line: &String) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn read_blocks(file: &String) -> Vec<Vec<String>> {
    lines_to_blocks(read_lines(file))
}

#[allow(dead_code)]
pub fn read_int_blocks(file: &String) -> Vec<Vec<i32>> {
    read_blocks(file)
        .iter()
        .map(|block| {
            block
                .iter()
                .map(|line| line.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}
