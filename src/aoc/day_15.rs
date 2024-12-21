struct Data {
    grid: Vec<Vec<char>>,
    moves: Vec<char>
}

impl Data {
    fn parse(data: &str) -> Self {
        let data = data.trim().replace("\r", "");
        let (grid, moves) = data.split_once("\n\n").unwrap();
        let grid = grid.lines().map(|s| {
            s.trim().chars().collect()
        }).collect();
        let moves = moves.lines().map(|s| s.trim()).collect::<String>().chars().collect();
        Data {grid, moves}
    }

    // fn get(&self, x: i32, y: i32) -> char {
    //     self.grid[y as usize][x as usize]
    // }

    // fn get_mut(&mut self, x: i32, y: i32) -> &mut char {
    //     &mut self.grid[y as usize][x as usize]
    // }

    fn size(&self) -> (i32, i32) {
        (self.grid[0].len() as i32, self.grid.len() as i32)
    }
}

fn get_tile(grid: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    grid[y as usize][x as usize]
}

fn set_tile(grid: &mut Vec<Vec<char>>, x: i32, y: i32, c: char) {
    grid[y as usize][x as usize] = c;
}

fn find_robot(data: &Data) -> Option<(i32, i32)> {
    let (w, h) = data.size();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if get_tile(&data.grid, x, y) == '@' {
                return Some((x, y));
            }
        }
    }
    None
}

fn part_1(data: &Data) -> u32 {
    let mut grid = data.grid.clone();

    let (mut x, mut y) = find_robot(&data).expect("robot shoud be present");

    for m in &data.moves {
        let (dx, dy) = match m {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("invalid move")
        };
        let (mut nx, mut ny) = (x + dx, y + dy);
        while get_tile(&grid, nx, ny) == 'O' {
            nx += dx;
            ny += dy;
        }
        if get_tile(&grid, nx, ny) == '.' {
            let d = (nx - x).abs() + (ny - y).abs();
            for _ in 0..d {
                let c = get_tile(&grid, nx - dx, ny - dy);
                set_tile(&mut grid, nx, ny, c);
                nx -= dx;
                ny -= dy;
            }
            set_tile(&mut grid, x, y, '.');
            x += dx; y += dy;
        }
    }
    let mut total = 0;

    let (w, h) = data.size();
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            if get_tile(&grid, x, y) == 'O' {
                total += x + 100 * y;
            }
        }
    }

    total as u32
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2");
    // let grid: Vec<Vec<char>> = data.grid.iter().map(|cs| {
    //     cs.iter().map(|c| {
    //         match c {
    //             '@' => "@.",
    //             '#' => "##",
    //             'O' => "[]",
    //             '.' => "..",
    //             _ => panic!("invalid tile")
    //         }
    //     }).collect()
    // }).map(|s: String| s.chars().collect()).collect();
}

pub fn solve() {
    let data = include_str!("../../data/day_15/input.txt");
    let data = Data::parse(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_15/test_1.txt");
        let data = Data::parse(data);
        assert!(data.grid.len() == 8);
        assert!(data.grid[0].len() == 8);
        assert!(data.moves.len() == 15);

        let data = include_str!("../../data/day_15/test_2.txt");
        let data = Data::parse(data);
        assert!(data.grid.len() == 10);
        assert!(data.grid[0].len() == 10);
        assert!(data.moves.len() == 700);
    }

    #[test]
    fn test_part_1_1() {
        let data = include_str!("../../data/day_15/test_1.txt");
        let data = Data::parse(data);
        assert!(part_1(&data) == 2028);
    }

    #[test]
    fn test_part_1_2() {
        let data = include_str!("../../data/day_15/test_2.txt");
        let data = Data::parse(data);
        assert!(part_1(&data) == 10092);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_15/test_1.txt");
        let data = Data::parse(data);
        assert!(part_2(&data) == 0);
    }
}
