use std::collections::HashSet;

struct Data {
    map: Vec<Vec<u8>>
}

impl Data {
    fn new(data: &str) -> Self {
        let map = data.trim().lines().map(|s| s.trim().chars().map(|c| c as u8).collect()).collect();
        Data {map}
    }
}

fn part_1(data: &Data) -> u32 {
    let (iw, ih) = (data.map[0].len() as i32, data.map.len() as i32);
    let mut map = data.map.clone();
    let mut factories = Vec::new();
    while let Some(((x, y), t)) = map.iter().enumerate().map(|(y, ts)| ts.iter().enumerate().map(move |(x, t)| ((x, y), *t))).flatten().find(|(_, t)| *t != 0) {
        let mut factory = HashSet::new();
        let mut ps = Vec::new();
        factory.insert((x as i32, y as i32));
        ps.push((x as i32, y as i32));
        while !ps.is_empty() {
            let (x, y) = ps.pop().unwrap();
            map[y as usize][x as usize] = 0;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = ((x as i32) + dx, (y as i32) + dy);
                if (nx >= 0) && (nx < iw) && (ny >= 0) && (ny < ih) && (map[ny as usize][nx as usize] == t) {
                    factory.insert((nx, ny));
                    ps.push((nx, ny));
                }
            }
        }
        factories.push((t, factory));
    }
    factories.iter().map(|(t, ps)| {
        let mut area = 0u32;
        let mut perimeter = 0u32;
        for (x, y) in ps {
            area += 1;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (nx, ny) = (x + dx, y + dy);
                if (nx < 0) || (nx == iw) || (ny < 0) || (ny == ih) || (data.map[ny as usize][nx as usize] != *t) {
                    perimeter += 1;
                }
            }
        }
        area * perimeter
    }).sum()
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2")
}

pub fn solve() {
    let data = include_str!("../../data/day_12/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_12/test.txt");
        let data = Data::new(data);
        assert!(data.map.len() == 10);
        assert!(data.map[0].len() == 10);
        assert!(data.map.iter().all(|v| v.len() == data.map[0].len()));
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_12/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 1930);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_12/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 1206);
    }
}
