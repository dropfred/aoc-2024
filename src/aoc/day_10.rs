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
        ps.insert((x, y));
        for z in 1..=9 {
            let mut nps = HashSet::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                if (((x as i32) - 1) >= 0) && (self.grid[y][x - 1] == z) {nps.insert((x - 1, y));}
                if (((x as i32) + 1) < w as i32) && (self.grid[y][x + 1] == z) {nps.insert((x + 1, y));}
                if (((y as i32) - 1) >= 0) && (self.grid[y - 1][x] == z) {nps.insert((x, y - 1));}
                if (((y as i32) + 1) < h as i32) && (self.grid[y + 1][x] == z) {nps.insert((x, y + 1));}
            }
            ps = nps;
        }
        ps.len() as u32
    }

    fn rating(&self, x: usize, y: usize) -> u32 {
        if self.grid[y][x] != 9 {return 0;}
        let (w, h) = (self.grid[0].len(), self.grid.len());
        let mut ps = Vec::new();
        ps.push((x, y));
        for z in 1..=9 {
            let z = 9 - z;
            let mut nps = Vec::new();
            for (x, y) in &ps {
                let (x, y) = (*x, *y);
                if (((x as i32) - 1) >= 0) && (self.grid[y][x - 1] == z) {nps.push((x - 1, y));}
                if (((x as i32) + 1) < w as i32) && (self.grid[y][x + 1] == z) {nps.push((x + 1, y));}
                if (((y as i32) - 1) >= 0) && (self.grid[y - 1][x] == z) {nps.push((x, y - 1));}
                if (((y as i32) + 1) < h as i32) && (self.grid[y + 1][x] == z) {nps.push((x, y + 1));}
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
