use std::collections::HashSet;

struct Data {
    grid: Vec<Vec<u8>>
}

impl Data {
    fn new(data: &str) -> Self {
        let grid = data.trim().lines().map(|s| s.trim().chars().map(|c| c.to_digit(10).unwrap_or(255) as u8).collect()).collect();
        Data {grid}
    }

    fn score(&self, x: usize, y: usize) -> u32 {
        if self.grid[y][x] != 0 {return 0;}
        let (w, h) = (self.grid[0].len(), self.grid.len());
        let mut ps = HashSet::new();
        ps.insert((x as i32, y as i32));
        for z in 1..=9 {
            let mut nps = HashSet::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < w as i32) && (ny >= 0) && (ny < h as i32) && (self.grid[ny as usize][nx as usize] == z) {
                        nps.insert((nx, ny));
                    }
                }
            }
            ps = nps;
        }
        ps.len() as u32
    }

    fn rating(&self, x: usize, y: usize) -> u32 {
        if self.grid[y][x] != 9 {return 0;}
        let (w, h) = (self.grid[0].len(), self.grid.len());
        let mut ps = Vec::new();
        ps.push((x as i32, y as i32));
        for z in (0..=8).rev() {
            let mut nps = Vec::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if (nx >= 0) && (nx < w as i32) && (ny >= 0) && (ny < h as i32) && (self.grid[ny as usize][nx as usize] == z) {
                        nps.push((nx, ny));
                    }
                }
            }
            ps = nps;
        }
        ps.len() as u32
    }
}

fn part_1(data: &Data) -> u32 {
    let (w, h) = (data.grid[0].len(), data.grid.len());
    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            total += data.score(x, y);
        }
    }
    return total;
}

fn part_2(data: &Data) -> u32 {
    let (w, h) = (data.grid[0].len(), data.grid.len());
    let mut total = 0;
    for y in 0..h {
        for x in 0..w {
            total += data.rating(x, y);
        }
    }
    return total;
}

pub fn solve() {
    let data = include_str!("../../data/day_10/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_10/test.txt");
        let data = Data::new(data);
        assert!(data.grid.len() == 8);
        assert!(data.grid.iter().all(|levels| levels.len() == 8));
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_10/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 36);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_10/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 81);
    }
}
