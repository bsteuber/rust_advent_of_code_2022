use crate::util;

#[derive(Debug, PartialEq)]
struct Trees {
    trees: Vec<Vec<i64>>,
    rows: usize,
    cols: usize,
}

impl Trees {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut trees = Vec::new();
        for line in lines {
            let mut current_row = Vec::new();
            for char in line.chars() {
                current_row.push(char.to_string().parse::<i64>().unwrap())
            }
            trees.push(current_row)
        }
        let rows = trees.len();
        let cols = trees[0].len();
        Self { trees, rows, cols }
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let height = self.trees[row][col];
        (0..row).all(|row| self.trees[row][col] < height)
            || (row + 1..self.rows).all(|row| self.trees[row][col] < height)
            || (0..col).all(|col| self.trees[row][col] < height)
            || (col + 1..self.cols).all(|col| self.trees[row][col] < height)
    }

    fn scenic_dir(&self, height: i64, tree_iter: impl Iterator<Item = (usize, usize)>) -> usize {
        let mut res = 0;
        for (row, col) in tree_iter {
            res += 1;
            if self.trees[row][col] >= height {
                return res;
            }
        }
        res
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let height = self.trees[row][col];
        let up = self.scenic_dir(height, (0..row).rev().map(|row| (row, col)));
        let down = self.scenic_dir(height, ((row + 1)..self.rows).map(|row| (row, col)));
        let left = self.scenic_dir(height, (0..col).rev().map(|col| (row, col)));
        let right = self.scenic_dir(height, ((col + 1)..self.cols).map(|col| (row, col)));
        up * down * left * right
    }
}

pub fn part_1(file: &str) -> usize {
    let contents = util::read_str(file);
    let lines = contents.lines();
    let trees = Trees::parse(lines);
    (0..trees.rows)
        .map(|row| {
            (0..trees.cols)
                .filter(|col| trees.is_visible(row, *col))
                .count()
        })
        .sum()
}

pub fn part_2(file: &str) -> usize {
    let contents = util::read_str(file);
    let lines = contents.lines();
    let trees = Trees::parse(lines);
    (0..trees.rows)
        .map(|row| {
            (0..trees.cols)
                .map(|col| trees.scenic_score(row, col))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::Trees;

    #[test]
    fn test_parse() {
        let lines = vec!["1234", "5678", "9876"];
        assert_eq!(
            Trees::parse(lines.iter().copied()),
            Trees {
                trees: vec!(vec!(1, 2, 3, 4), vec!(5, 6, 7, 8), vec!(9, 8, 7, 6)),
                rows: 3,
                cols: 4
            }
        );
    }
}
