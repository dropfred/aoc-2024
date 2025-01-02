use std::collections::VecDeque;

#[derive(Clone)]
pub struct Grid<T> {
    size: (usize, usize),
    cells: Vec<T>
}

impl<T: Copy + std::cmp::PartialEq> Grid<T> {
    pub fn new(size: (usize, usize), e: T) -> Self {
        let s = (size.0 * size.1);
        assert!(s > 0);
        let cells = vec![e; s];
        Self {size, cells}
    }

    pub fn load(data: &Vec<Vec<T>>) -> Self {
        debug_assert!(!data.is_empty() && data.iter().all(|r| !r.is_empty() && (r.len() == data[0].len())));
        let (width, height) = (data[0].len(), data.len());
        let mut cells = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.push(data[y][x]);
            }
        }
        Self {size: (width, height), cells}
    }

    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get(&self, point: (usize, usize)) -> T {
        self.cells[point.1 * self.size.0 + point.0]
    }

    pub fn set(&mut self, point: (usize, usize), v: T) {
        self.cells[point.1 * self.size.0 + point.0] = v;
    }

    pub fn find(&self, v: T) -> Option<(usize, usize)> {
        let (w, h) = self.size();
        for y in 0..h {
            for x in 0..w {
                if self.get((x, y)) == v {
                    return Some((x, y));
                }
            }
        }
        None
    }


    pub fn explore<S: FnMut((usize, usize), T) -> bool>(&self, start: (usize, usize), step: S) -> GridExploreIterator<T, S> {
        GridExploreIterator::new(self, start, step)
    }
}

impl<T: Copy + std::cmp::PartialEq + std::str::FromStr + std::fmt::Debug> Grid<T> {
    pub fn parse(data: &str, sep: &str) -> Option<Self> where <T as std::str::FromStr>::Err: std::fmt::Debug {
        let cells: Vec<Vec<_>> = data.trim().lines().map(|r| {
            if sep.is_empty() {
                r.trim().chars().map(|c| c.to_string().parse::<T>()).collect()
            } else {
                r.trim().split(sep).map(|s| s.parse::<T>()).collect()
            }
        }).collect();
        if cells.is_empty() || cells.iter().any(|r| r.iter().any(|c| c.is_err())) {
            return None;
        }
        let cells = cells.into_iter().map(|r| r.into_iter().map(|v| v.unwrap()).collect()).collect();
        Some(Self::load(&cells))
    }
}

enum Dir {
    East,
    West,
    North,
    South
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::East => (-1, 0),
            Dir::West => (1, 0),
            Dir::North => (0, -1),
            Dir::South => (0, 1)
        }
    }

    fn all() -> [Dir; 4] {
        [Dir::East, Dir::West, Dir::North, Dir::South]
    }
}

pub struct GridExploreIterator<'a, T, S: FnMut((usize, usize), T) -> bool> {
    grid: &'a Grid<T>,
    step: S,
    positions: VecDeque<((usize, usize), usize)>,
    visited: Vec<u64>
}

impl<'a, T: Copy + std::cmp::PartialEq, S: FnMut((usize, usize), T) -> bool> GridExploreIterator<'a, T, S> {
    pub fn new(grid: &'a Grid<T>, start: (usize, usize), step: S) -> Self {
        let (w, h) = grid.size();
        let positions = VecDeque::new();
        let visited = vec![0; h * (w + 63) / 64];
        let mut it = GridExploreIterator {grid, step, positions, visited};
        it.visit(start, 0);
        it
    }

    fn visit(&mut self, position: (usize, usize), distance: usize) {
        let (w, _) = self.grid.size();
        let w64 = (w + 63) / 64;
        let (vx, vy) = (position.0 / 64, position.1);
        let bx = 1 << (position.0 % 64);
        let v = vy * w64 + vx;
        if (self.visited[v] & bx) == 0 {
            if (self.step)(position, self.grid.get(position)) {
                self.visited[v] |= bx;
                self.positions.push_back((position, distance));
            }
        }
    }
}

impl<'a, T: Copy + std::cmp::PartialEq, S: FnMut((usize, usize), T) -> bool> Iterator for GridExploreIterator<'a, T, S> {
    type Item = ((usize, usize), usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((position, distance)) = self.positions.pop_front() {
            let (w, h) = {let (w, h) = self.grid.size(); (w as i32, h as i32)};
            let (x, y) = (position.0 as i32, position.1 as i32);
            for d in Dir::all() {
                let (dx, dy) = d.offset();
                let (x, y) = (x + dx, y + dy);
                if (x >= 0) && (x < w) && (y >= 0) && (y < h) {
                    let p = (x as usize, y as usize);
                    self.visit(p, distance + 1);
                }
            }
            Some((position, distance))
        } else {
            None
        }
    }
}

impl<> std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for y in 0..(self.size.1) {
            if nl {
                writeln!(f, " ")?;
            }
            for x in 0..(self.size.0) {
                write!(f, "{}", self.get((x, y)))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "
        1 2 3
        4 5 6
        7 8 9
        ";
        let grid: Grid<u8> = Grid::parse(data, " ").unwrap();
        assert_eq!(grid.size(), (3, 3));
        assert_eq!(grid.get((0, 0)), 1);
        assert_eq!(grid.get((2, 2)), 9);

        let data = "
        abc
        def
        ";
        let grid: Grid<char> = Grid::parse(data, "").unwrap();
        assert_eq!(grid.size(), (3, 2));

        let data = "
        123
        456
        789
        ";
        let grid: Grid<u8> = Grid::parse(data, "").unwrap();
        assert_eq!(grid.size(), (3, 3));
        let grid: Grid<u32> = Grid::parse(data, ",").unwrap();
        assert_eq!(grid.size(), (1, 3));

        let data = "
        1X3
        456
        ";
        let grid: Option<Grid<u8>> = Grid::parse(data, "");
        assert!(grid.is_none());
    }
}
