use crate::util;

type Num = i64;

#[derive(Clone)]
struct EncryptedFile {
    items: Vec<Num>,
    positions: Vec<usize>,
}

impl EncryptedFile {
    fn new(items: Vec<Num>) -> Self {
        let positions = (0..items.len()).collect();
        Self { items, positions }
    }

    fn parse(file: &str) -> Self {
        let items: Vec<Num> = util::read_lines(file)
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
        Self::new(items)
    }

    fn find_zero(&self) -> usize {
        let id = self
            .items
            .iter()
            .position(|i| *i == 0)
            .expect("Item 0 not found!");
        self.find_position(id)
    }

    fn find_position(&self, id: usize) -> usize {
        self.positions
            .iter()
            .position(|p| *p == id)
            .expect("Position of id not found")
    }

    fn as_position(&self, position: Num) -> usize {
        let len = self.items.len() as Num;
        (((position as Num % len) + len) % len) as usize
    }

    fn end_position(&self, start: usize, item: Num) -> usize {
        let len = self.items.len() as Num;
        let mut pos = start as Num + item;
        if pos >= len {
            pos = pos % (len - 1);
        }
        if pos <= 0 {
            pos = (pos % (len - 1)) + (len - 1);
        }

        pos as usize
    }

    fn move_right(&mut self, id: usize, start: usize, end: usize) {
        for i in start..end {
            self.positions[i] = self.positions[i + 1]
        }
        self.positions[end] = id
    }

    fn move_left(&mut self, id: usize, start: usize, end: usize) {
        for i in (end..start).rev() {
            self.positions[i + 1] = self.positions[i]
        }
        self.positions[end] = id
    }

    fn move_item(&mut self, id: usize) {
        let item = self.items[id];
        if item != 0 {
            let start = self.find_position(id);
            let end = self.end_position(start, item);
            // println!("\nbefore: {:?}", self.items);
            if end > start {
                // println!("Move left: item {} from {} to {}", item, start, end);
                self.move_right(id, start, end)
            } else if start > end {
                // println!("Move right: item {} from {} to {}", item, start, end);
                self.move_left(id, start, end)
            }
        }
        // println!("after: {:?}\n", self.items);
    }

    fn move_all(&mut self) {
        for id in 0..self.items.len() {
            // println!("Move item {id}");
            self.move_item(id)
        }
    }

    fn get_item(&self, position: Num) -> Num {
        self.items[self.positions[self.as_position(position)]]
    }

    fn get_coordinates(&self) -> Num {
        let pos_0 = self.find_zero() as Num;
        let mut res = 0 as Num;
        for add_pos in [1000, 2000, 3000] {
            let item = self.get_item(pos_0 + add_pos);
            println!("Item at relative position {} is {}", add_pos, item);
            res += item;
        }
        res
    }

    fn apply_decryption_key(&mut self, key: Num) {
        for i in 0..self.items.len() {
            self.items[i] *= key
        }
    }
}

fn part_1(mut f: EncryptedFile) {
    f.move_all();
    println!("Part 1: {}", f.get_coordinates());
}

fn part_2(mut f: EncryptedFile) {
    f.apply_decryption_key(811589153);
    for _mix in 0..10 {
        // println!("Mix {mix}");
        f.move_all();
    }
    println!("Part 2: {}", f.get_coordinates());
}

pub fn run() {
    // let f = EncryptedFile::parse("20-test");
    let f = EncryptedFile::parse("20-input");
    part_1(f.clone());
    part_2(f.clone());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_right() {
        let mut f = EncryptedFile::new(vec![1, 2, 3, 8]);
        f.move_item(0);
        assert_eq!(f.positions, vec![0, 1, 2, 3]);
        f.move_item(2);
        assert_eq!(f.items, vec![2, 1, 3, 8]);
        f.move_item(3);
        assert_eq!(f.items, vec![2, 1, 8, 3]);
    }

    #[test]
    fn test_move_left() {
        let mut f = EncryptedFile::new(vec![0, -1, -2, -9]);
        f.move_item(0);
        assert_eq!(f.items, vec![0, -1, -2, -9]);
        f.move_item(1);
        assert_eq!(f.items, vec![0, -2, -9, -1]);
        f.move_item(2);
        assert_eq!(f.items, vec![0, -9, -2, -1]);
        f.move_item(3);
        assert_eq!(f.items, vec![0, -9, -2, -1]);
    }

    // #[test]
    // fn test_duplicates() {
    //     let mut f =
    //         EncryptedFile::new(vec![1 /*a*/, 2 /*a*/, 1 /*b*/, 2 /*b*/]);
    // }
}
