use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque, HashMap, HashSet}};

#[derive(Clone)]
struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>
}

impl<T: Copy + std::cmp::PartialEq> Grid<T> {
    fn new(width: usize, height: usize, e: T) -> Self {
        let size = width * height;
        assert!(size > 0);
        let cells = vec![e; size];
        Self {width, height, cells}
    }

    fn load(data: &Vec<Vec<T>>) -> Self {
        debug_assert!(!data.is_empty() && data.iter().all(|r| !r.is_empty() && (r.len() == data[0].len())));
        let (width, height) = (data[0].len(), data.len());
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(data[y][x]);
            }
        }
        Self {width, height, cells}
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get(&self, x: usize, y: usize) -> T {
        self.cells[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, v: T) {
        self.cells[y * self.width + x] = v;
    }

    fn find(&self, v: T) -> Option<(usize, usize)> {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                if self.get(x, y) == v {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl<> std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for y in 0..(self.height) {
            if nl {
                writeln!(f, "")?;
            }
            for x in 0..(self.width) {
                write!(f, "{}", self.get(x, y))?;
            }
            nl = true;
        }
        Ok(())
    }
}

// impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let mut nl = false;
//         for y in 0..(self.height) {
//             if nl {
//                 writeln!(f, "")?;
//             }
//             for x in 0..(self.width) {
//                 write!(f, "{}", self.get(x, y))?;
//             }
//             nl = true;
//         }
//         Ok(())
//     }
// }

struct Maze {
    map: Grid<char>
}

const DIRS: &[char; 4] = &['<', '>', '^', 'v'];

impl Maze {
    fn parse(data: &str) -> Option<Self> {
        let cells: Vec<Vec<_>> = data.trim().lines().map(|r| {
            r.trim().chars().collect()
        }).collect();
        if cells.is_empty() || cells.iter().any(|r| {r.is_empty() || (r.len() != cells[0].len())}) {
            return None;
        }
        let cells = cells.into_iter().collect();
        let map = Grid::load(&cells);
        Some(Self {map})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn is_valid(&self, x: usize, y: usize, wall: char) -> bool {
        let (w, h) = self.map.size();
        (x < w) && (y < h) && (self.map.get(x, y) != wall)
    }

    fn get_distances_from(&self, begin: (usize, usize), wall: char) -> Grid<usize> {
        let pb = (begin.0 as i32, begin.1 as i32);
        let mut ds = Grid::new(self.map.width, self.map.height, usize::MAX);
        let mut ps = VecDeque::new();
        let mut vs = HashSet::new();
        let mut visit = |ps: &mut VecDeque<_>, p, d: usize| {
            if !vs.contains(&p) {
                let (x, y) = p;
                if self.is_valid(x as usize, y as usize, wall) {
                    vs.insert(p);
                    ps.push_back((d, p));
                }
            }
        };
        visit(&mut ps, pb, 0);
        while !ps.is_empty() {
            let (d, p) = ps.pop_front().unwrap();
            let (x, y) = p;
            ds.set(x as usize, y as usize, d);
            for n in DIRS {
                let (dx, dy) = get_offset(*n);
                let (nx, ny) = (x + dx, y + dy);
                visit(&mut ps, (nx, ny), d + 1);
            }
        }
        ds
    }

    fn get_distance_between(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> Option<usize> {
        let pb = (begin.0 as i32, begin.1 as i32);
        let pe = (end.0 as i32, end.1 as i32);
        let mut ps = VecDeque::new();
        let mut vs = HashSet::new();
        let is_valid = |x, y| (x >= 0) && (y >= 0) && self.is_valid(x as usize, y as usize, wall);
        let mut visit = |ps: &mut VecDeque<_>, p, d: usize| {
            if !vs.contains(&p) {
                let (x, y) = p;
                if is_valid(x, y) {
                    vs.insert(p);
                    ps.push_back((d, p));
                }
            }
        };
        visit(&mut ps, pb, 0);
        while !ps.is_empty() {
            let (d, p) = ps.pop_front().unwrap();
            if p == pe {
                return Some(d);
            }
            let (x, y) = p;
            for n in DIRS {
                let (dx, dy) = get_offset(*n);
                let (nx, ny) = (x + dx, y + dy);
                visit(&mut ps, (nx, ny), d + 1);
            }
        }
        None
    }

    fn get_path_between(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> Option<Vec<(usize, usize)>> {
        let pb = (begin.0 as i32, begin.1 as i32);
        let pe = (end.0 as i32, end.1 as i32);
        let mut ps = VecDeque::new();
        let mut vs = HashSet::new();
        let is_valid = |x, y| (x >= 0) && (y >= 0) && self.is_valid(x as usize, y as usize, wall);
        let mut visit = |ps: &mut VecDeque<_>, path: Vec<(i32, i32)>| {
            let p = *path.last().unwrap();
            if !vs.contains(&p) {
                let (x, y) = p;
                if is_valid(x, y) {
                    vs.insert(p);
                    ps.push_back(path);
                }
            }
        };
        visit(&mut ps, vec![pb]);
        while !ps.is_empty() {
            let path = ps.pop_front().unwrap();
            let p: (i32, i32) = *path.last().unwrap();
            if p == pe {
                let path = path.iter().map(|(x, y)| (*x as usize, *y as usize)).collect();
                return Some(path);
            } else {
    
            }
            let (x, y) = p;
            for d in DIRS {
                let (dx, dy) = get_offset(*d);
                let (nx, ny) = (x + dx, y + dy);
                let mut path = path.clone();
                path.push((nx, ny));
                visit(&mut ps, path);
            }
        }
        None
    }
}

fn get_offset(d: char) -> (i32, i32) {
    match d {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => panic!("invalid direction")
    }
}

struct Puzzle {
    maze: Maze
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        Some(Self {maze: Maze::parse(data)?})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_cheats(maze: &Maze, begin: (usize, usize), end: (usize, usize), wall: char) -> Vec<((usize, usize), usize)> {
    let ds = maze.get_distances_from(end, wall);
    let distance = ds.get(begin.0, begin.1);
    let mut cheats = HashMap::new();
    let begin = (begin.0 as i32, begin.1 as i32);
    let end = (end.0 as i32, end.1 as i32);
    let mut ps = VecDeque::new();
    let mut vs = HashSet::new();
    let is_valid = |x, y| (x >= 0) && (y >= 0) && maze.is_valid(x as usize, y as usize, wall);
    let mut visit = |ps: &mut VecDeque<_>, p, d: usize| {
        if !vs.contains(&p) {
            let (x, y) = p;
            if is_valid(x, y) {
                vs.insert(p);
                ps.push_back((d, p));
            }
        }
    };
    let cheat = |x, y, d| {
        if is_valid(x, y) {
            let de = ds.get(x as usize, y as usize);
            if de != usize::MAX {
                let d = d + de + 2;
                if d < distance {
                    return distance - d;
                }
            }
        }
        0
    };
    visit(&mut ps, begin, 0);
    while !ps.is_empty() {
        let (d, p) = ps.pop_front().unwrap();
        if p == end {
            // return Some(s);
            break;
        }
        let (x, y) = p;
        for n in DIRS {
            let (dx, dy) = get_offset(*n);
            let (nx, ny) = (x + dx, y + dy);
            visit(&mut ps, (nx, ny), d + 1);
            if maze.map.get(nx as usize, ny as usize) == wall {
                if !cheats.contains_key(&(nx, ny)) {
                    let mut ms = 0;
                    let (cx, cy) = (nx + dx, ny + dy);
                    let s = cheat(cx, cy, d);
                    if s > ms {ms = s;}
                    let (ax, ay) = (1 - dx.abs(), 1 - dy.abs());
                    let (cx, cy) = (nx - ax, ny - ay);
                    let s = cheat(cx, cy, d);
                    if s > ms {ms = s;}
                    let (cx, cy) = (nx + ax, ny + ay);
                    let s = cheat(cx, cy, d);
                    if s > ms {ms = s;}

                    cheats.insert((nx, ny), ms);
                }
            }
        }
    }
    cheats.into_iter().map(|((x, y), s)| ((x as usize, y as usize), s))
                      .filter(|(_, s)| *s > 0)
                      .collect()
}

fn solve_part_1(puzzle: &Puzzle, min: usize) -> usize {
    let maze = &puzzle.maze;
    let begin = puzzle.maze.map.find('S').unwrap();
    let end = puzzle.maze.map.find('E').unwrap();
    let cheats = get_cheats(maze, begin, end, '#');
    cheats.into_iter().filter(|(_, s)| *s >= min).count()
}

fn part_1(puzzle: &Puzzle) -> usize {
    solve_part_1(puzzle, 100)
}

fn part_2(puzzle: &Puzzle) -> (u32, u32) {
    todo!("part 2");
}

pub fn solve() {
    let puzzle = include_str!("../../data/day_20/input.txt");
    let puzzle = Puzzle::load(puzzle);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {:?}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let puzzle = include_str!("../../data/day_20/test.txt");
        let puzzle = Puzzle::load(puzzle);
        assert_eq!(solve_part_1(&puzzle, 1), 44);
        let maze = &puzzle.maze;
        let begin = maze.map.find('S').unwrap();
        let end = maze.map.find('E').unwrap();
        let cheats = get_cheats(maze, begin, end, '#');
        let mut map: HashMap<usize, usize> = HashMap::new();
        for (_, s) in cheats {
            let c = map.entry(s).or_insert(0);
            *c += 1;
        }
        let mut cheats: Vec<_> = map.into_iter().collect();
        cheats.sort();
        let mut i = cheats.into_iter();
        assert_eq!(i.next(), Some((2, 14)));
        assert_eq!(i.next(), Some((4, 14)));
        assert_eq!(i.next(), Some((6, 2)));
        assert_eq!(i.next(), Some((8, 4)));
        assert_eq!(i.next(), Some((10, 2)));
        assert_eq!(i.next(), Some((12, 3)));
        assert_eq!(i.next(), Some((20, 1)));
        assert_eq!(i.next(), Some((36, 1)));
        assert_eq!(i.next(), Some((38, 1)));
        assert_eq!(i.next(), Some((40, 1)));
        assert_eq!(i.next(), Some((64, 1)));
        assert_eq!(i.next(), None);
    }

    // #[test]
    // fn test_part_2() {
    //     let puzzle = include_str!("../../data/day_20/test.txt");
    //     let puzzle = Puzzle::load(puzzle);
    //     // assert!(solve_part_2(&puzzle, 7, 7, 12).unwrap() == (6, 1));
    // }
}
