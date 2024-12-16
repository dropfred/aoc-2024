use std::collections::{HashMap, HashSet};

struct Data {
    grid: Vec<Vec<char>>
}

impl Data {
    fn new(data: &str) -> Self {
        Data {grid: data.trim().lines().map(|s| s.trim().chars().collect()).collect()}
    }
}

fn part_1(data: &Data) -> u32 {
    let width = data.grid[0].len() as i32;
    let height = data.grid.len() as i32;
    let mut nodes = HashMap::new();
    let mut anti_nodes = HashSet::new();
    let mut count = 0u32;
    for y in 0..height {
        for x in 0..width {
            let c = data.grid[y as usize][x as usize];
            if c != '.' {
                let locs = nodes.entry(c).or_insert(Vec::new());
                for (lx, ly) in locs.iter() {
                    let (dx, dy) = (x - lx, y - ly);
                    let (nx, ny) = (lx - dx, ly - dy);
                    if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                        if !anti_nodes.contains(&(nx, ny)) {
                            anti_nodes.insert((nx, ny));
                            count += 1;
                        }
                    }
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < width) && (ny >= 0) && (ny < height) {
                        if !anti_nodes.contains(&(nx, ny)) {
                            anti_nodes.insert((nx, ny));
                            count += 1;
                        }
                    }
                }
                locs.push((x, y));
            }
        }
    }
    count
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_8/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_8/test.txt");
        let data = Data::new(data);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_8/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 14);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_8/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 14);
    }
}
