// https://dev.to/wrongbyte/implementing-iterator-and-intoiterator-in-rust-3nio

struct Data {
    letters: Vec<String>,
    size: (usize, usize),
}

impl Data {
    fn new(data: &str) -> Self {
        let letters: Vec<String> = data.lines().map(|s| s.to_string()).collect();
        let size = (letters[0].len(), letters.len());
        Data {letters, size}
    }

    fn get(&self, r: usize, c: usize) -> char {
        self.letters[r].as_bytes()[c] as char
    }
}

struct DataIterator<'a> {
    data: &'a Data,
    position: usize,
    direction: usize
}

impl<'a> DataIterator<'a> {
    fn next_direction(&mut self) {
        // println!("+ {} / {}", self.direction, self.position);
        self.position = 0;
        self.direction += 1;
    }

    fn next_position(&mut self) {
        self.position += 1;
        match self.direction {
            0 => {
                if self.position == self.data.size.1 {
                    self.next_direction();
                }
            },
            1 => {
                if self.position == self.data.size.0 {
                    self.next_direction();
                }
            },
            2..=3 => {
                if self.position == ((self.data.size.0 + self.data.size.1) - 1) {
                    self.next_direction();
                }
            },
            _ => ()
        }
    }
}

impl<'a> IntoIterator for &'a Data {
    type Item = String;
    type IntoIter = DataIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DataIterator {data: self, position: 0, direction: 0}
    }
}

impl<'a> Iterator for DataIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            0 => {
                let r = self.data.letters[self.position].clone();
                self.next_position();
                Some(r)
            },
            1 => {
                let mut r = String::new();
                for s in &self.data.letters {
                    r.push(s.as_bytes()[self.position] as char);
                }
                self.next_position();
                Some(r)
            },
            2 => {
                let mut r = String::new();
                let br; let bc;
                if self.position < self.data.size.1 {
                    br = self.position;
                    bc = 0;
                } else {
                    br = self.data.size.1 - 1;
                    bc = self.position - self.data.size.1 + 1;
                }
                let s = std::cmp::min(br + 1, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br - i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            },
            3 => {
                let mut r = String::new();
                let br; let bc;
                if self.position < self.data.size.1 {
                    br = (self.data.size.1 - 1) - self.position;
                    bc = 0;
                } else {
                    br = 0;
                    bc = self.position - self.data.size.1 + 1;
                }
                let s = std::cmp::min(self.data.size.1 - br, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br + i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            },
            _ => None
        }
    }
}

/*
pub struct DataIntoIterator {
    data: Data,
    position: usize,
    direction: usize
}

impl DataIntoIterator {
    fn next_direction(&mut self) {
        self.position = 0;
        self.direction += 1;
    }

    fn next_position(&mut self) {
        self.position += 1;
        match self.direction {
            0 => {
                if self.position == self.data.size.1 {
                    self.next_direction();
                }
            },
            1 => {
                if self.position == self.data.size.0 {
                    self.next_direction();
                }
            },
            2..=3 => {
                if self.position == ((self.data.size.0 + self.data.size.1) - 1) {
                    self.next_direction();
                }
            },
            _ => ()
        }
    }
}

impl IntoIterator for Data {
    type Item = String;
    type IntoIter = DataIntoIterator;

    fn into_iter(self) -> DataIntoIterator {
        DataIntoIterator {data: self, position: 0, direction: 0}
    }
}

impl Iterator for DataIntoIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            0 => {
                let r = self.data.letters[self.position].clone();
                self.next_position();
                Some(r)
            },
            1 => {
                let mut r = String::new();
                for s in &self.data.letters {
                    r.push(s.as_bytes()[self.position] as char);
                }
                self.next_position();
                Some(r)
            },
            2 => {
                let mut r = String::new();
                let br; let bc;
                if self.position < self.data.size.1 {
                    br = self.position;
                    bc = 0;
                } else {
                    br = self.data.size.1 - 1;
                    bc = self.position - self.data.size.1 + 1;
                }
                let s = std::cmp::min(br + 1, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br - i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            },
            3 => {
                let mut r = String::new();
                let br; let bc;
                if self.position < self.data.size.1 {
                    br = (self.data.size.1 - 1) - self.position;
                    bc = 0;
                } else {
                    br = 0;
                    bc = self.position - self.data.size.1 + 1;
                }
                let s = std::cmp::min(self.data.size.1 - br, self.data.size.0 - bc);
                for i in 0..s {
                    r.push(self.data.letters[br + i].as_bytes()[bc + i] as char);
                }
                self.next_position();
                Some(r)
            },
            _ => None
        }
    }
}
*/

fn part_1(data: &Data) -> u32 {
    let mut total = 0;
    for s in data {
        let nf = s.match_indices("XMAS").count();
        let nb = s.match_indices("SAMX").count();
        total += nf + nb;
    }
    total as u32
}

fn part_2(data: &Data) -> u32 {
    let bc = 1;
    let ec = data.size.0 - 1;
    let br = 1;
    let er = data.size.1 - 1;
    let mut total = 0;

    for r in br..er {
        for c in bc..ec {
            if data.get(r, c) == 'A' && ((data.get(r - 1, c - 1) == 'M' && data.get(r + 1, c + 1) == 'S') || (data.get(r - 1, c - 1) == 'S' && data.get(r + 1, c + 1) == 'M')) && ((data.get(r - 1, c + 1) == 'M' && data.get(r + 1, c - 1) == 'S') || (data.get(r - 1, c + 1) == 'S' && data.get(r + 1, c - 1) == 'M')) {
                total += 1;
            }
        }
    }
    total
}

pub fn solve() {
    let data = include_str!("../../data/day_4/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_4/test.txt");

    #[test]
    fn test_data() {
        let data = Data::new(DATA);
        assert!(data.size.1 == 10);
        assert!(data.letters.len() == data.size.1);
        assert!(data.size.0 == 10);
        assert!(data.letters.iter().all(|line| line.len() == data.size.0));
    }

    #[test]
    fn test_part_2() {
        let data = Data::new(DATA);
        assert!(part_2(&data) == 9);
    }
}
