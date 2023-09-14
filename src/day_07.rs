use std::collections::HashMap;

use crate::util;

#[derive(Debug)]
struct Directory {
    sub_dirs: HashMap<String, Directory>,
    file_sizes: HashMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            sub_dirs: HashMap::new(),
            file_sizes: HashMap::new(),
        }
    }
}

struct FileSystem {
    root: Directory,
    path: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: Directory::new(),
            path: Vec::new(),
        }
    }
    fn current_dir(&mut self) -> &mut Directory {
        let mut dir = &mut self.root;
        for dir_name in &self.path {
            dir = dir.sub_dirs.get_mut(dir_name).expect("Dir not found");
        }
        dir
    }

    fn apply_cd(&mut self, dir: &str) {
        match dir {
            "/" => self.path = Vec::new(),
            ".." => {
                self.path.pop().unwrap();
            }
            _ => self.path.push(dir.to_string()),
        }
    }

    fn add_dir(&mut self, name: &str) {
        self.current_dir()
            .sub_dirs
            .insert(name.to_string(), Directory::new());
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.current_dir().file_sizes.insert(name.to_string(), size);
    }

    fn apply_line(&mut self, line: &str) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "$" => {
                if tokens[1] == "cd" {
                    self.apply_cd(tokens[2])
                }
            }
            "dir" => self.add_dir(tokens[1]),
            size_str => self.add_file(tokens[1], size_str.parse().unwrap()),
        }
    }

    fn list_total_sizes(&self) -> Vec<usize> {
        let mut res = Vec::new();
        fn rec_list(dir: &Directory, res: &mut Vec<usize>) -> usize {
            let file_sum: usize = dir.file_sizes.iter().map(|(_, size)| size).sum();
            let dir_sum: usize = dir
                .sub_dirs
                .iter()
                .map(|(_, sub_dir)| rec_list(sub_dir, res))
                .sum();
            let sum = file_sum + dir_sum;
            res.push(sum);
            sum
        }
        rec_list(&self.root, &mut res);
        res
    }
}

pub fn part_1(file: &str) -> usize {
    let mut file_system = FileSystem::new();
    for line in util::read_lines(file) {
        file_system.apply_line(&line);
    }
    //println!("File Sys: {:#?}", file_system.root);
    let sizes = file_system.list_total_sizes();
    sizes.iter().filter(|size| **size < (100000 as usize)).sum()
}

pub fn part_2(file: &str) -> usize {
    let mut file_system = FileSystem::new();
    for line in util::read_lines(file) {
        file_system.apply_line(&line);
    }
    //println!("File Sys: {:#?}", file_system.root);
    let sizes = file_system.list_total_sizes();
    let total_used: usize = *sizes.last().unwrap();
    let total_space = 70000000;
    let free_space = total_space - total_used;
    let required_space = 30000000;
    let need_to_delete = required_space - free_space;
    *sizes
        .iter()
        .filter(|size| **size >= need_to_delete)
        .min()
        .unwrap()
}
