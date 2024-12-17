// use std::collections::{HashMap, HashSet};

struct Data {
    fs: Vec<u32>
}

impl Data {
    fn new(data: &str) -> Self {
        // Data {grid: data.trim().lines().map(|s| s.trim().chars().collect()).collect()}
        let mut fs = Vec::new();
        for (i, c) in data.trim().as_bytes().iter().enumerate() {
            let n = c - b'0';
            let id = if (i & 1) == 0 {(i / 2) as u32} else {u32::MAX};
            fs.extend(std::iter::repeat(id).take(n as usize));
        }
        Data {fs}
    }
}

fn part_1(data: &Data) -> u64 {
    let mut fs = data.fs.clone();
    let mut b = 0;
    let mut e = fs.len() - 1;
    loop {
        while (b < e) && (fs[b] != u32::MAX) {b += 1;}
        while (e > b) && (fs[e] == u32::MAX) {e -= 1;}
        if b < e {
            fs[b] = fs[e];
            fs[e] = u32::MAX;
            b += 1;
            e -= 1;
        } else {
            break;
        }
    }
    let mut checksum = 0;
    for (i, id) in fs.iter().enumerate() {
        if *id == u32::MAX {break;}
        checksum += (i as u64) * (*id as u64);
    }
    checksum
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2")
}

pub fn solve() {
    let data = include_str!("../../data/day_9/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        assert!(data.fs.len() == "00...111...2...333.44.5555.6666.777.888899".len());
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 1928);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 2858);
    }
}
