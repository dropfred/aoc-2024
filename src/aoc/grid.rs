use std::collections::VecDeque;

#[derive(Clone)]
pub struct Grid<T> {
    size: (usize, usize),
    cells: Vec<T>
}

impl<T: Copy + std::cmp::PartialEq> Grid<T> {
    pub fn new(size: (usize, usize), e: T) -> Self {
        let s = size.0 * size.1;
        assert!(s > 0);
        let cells = vec![e; s];
        Self {size, cells}
    }

    pub fn from_vec(data: &Vec<Vec<T>>) -> Self {
        assert!(!data.is_empty() && !data[0].is_empty() && data.iter().skip(1).all(|r| r.len() == data[0].len()));
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

    pub fn explore<F: FnMut((usize, usize), (usize, usize), usize) -> bool>(&self, start: (usize, usize), filter: F) -> GridExploreIterator<T, F> {
        GridExploreIterator::new(self, start, filter)
    }
}

impl<T: Copy + std::cmp::PartialEq + std::str::FromStr + std::fmt::Debug> Grid<T> {
    pub fn parse(data: &str, sep: &str) -> Option<Self> where <T as std::str::FromStr>::Err: std::fmt::Debug {
        let cells: Result<Vec<Vec<_>>, _> = data.trim().lines().map(|r| {
            if sep.is_empty() {
                r.trim().chars().map(|c| T::from_str(&c.to_string())).collect()
            } else {
                r.trim().split(sep).map(|s| T::from_str(&s)).collect()
            }
        }).collect();
        let cells = cells.ok()?;
        if !cells.is_empty() && !cells[0].is_empty() && cells.iter().skip(1).all(|r| r.len() == cells[0].len()) {
            Some(Self::from_vec(&cells))
        } else {
            None
        }
    }

    pub fn load(data: &str, sep: &str) -> Self where <T as std::str::FromStr>::Err: std::fmt::Debug {
        Self::parse(data, sep).expect("valid input")
    }
}

impl std::fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut nl = false;
        for y in 0..(self.size.1) {
            if nl {
                writeln!(f, "")?;
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

pub struct GridExploreIterator<'a, T, F: FnMut((usize, usize), (usize, usize), usize) -> bool> {
    grid: &'a Grid<T>,
    filter: F,
    positions: VecDeque<((usize, usize), (usize, usize), usize)>,
    visited: Vec<u64>
}

impl<'a, T: Copy + std::cmp::PartialEq, F: FnMut((usize, usize), (usize, usize), usize) -> bool> GridExploreIterator<'a, T, F> {
    pub fn new(grid: &'a Grid<T>, start: (usize, usize), filter: F) -> Self {
        let (w, h) = grid.size();
        let positions = VecDeque::new();
        let visited = vec![0; h * (w + 63) / 64];
        let mut it = GridExploreIterator {grid, filter, positions, visited};
        it.visit(start, start, 0);
        it
    }

    fn visit(&mut self, position: (usize, usize), pposition: (usize, usize), distance: usize) {
        let (w, _) = self.grid.size();
        let w64 = (w + 63) / 64;
        let (vx, vy) = (position.0 / 64, position.1);
        let bx = 1 << (position.0 % 64);
        let v = vy * w64 + vx;
        if (self.visited[v] & bx) == 0 {
            self.visited[v] |= bx;
            if (self.filter)(position, pposition, distance) {
                self.positions.push_back((position, pposition, distance));
            }
        }
    }
}

impl<'a, T: Copy + std::cmp::PartialEq, F: FnMut((usize, usize), (usize, usize), usize) -> bool> Iterator for GridExploreIterator<'a, T, F> {
    type Item = ((usize, usize), (usize, usize), usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((position, pposition, distance)) = self.positions.pop_front() {
            let (w, h) = {let (w, h) = self.grid.size(); (w as i32, h as i32)};
            let (x, y) = (position.0 as i32, position.1 as i32);
            for d in Dir::all() {
                let (dx, dy) = d.offset();
                let (x, y) = (x + dx, y + dy);
                if (x >= 0) && (x < w) && (y >= 0) && (y < h) {
                    let p = (x as usize, y as usize);
                    self.visit(p, position, distance + 1);
                }
            }
            Some((position, pposition, distance))
        } else {
            None
        }
    }
}

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
        let grid: Grid<u8> = Grid::load(data, " ");
        assert_eq!(grid.size(), (3, 3));
        assert_eq!(grid.get((0, 0)), 1);
        assert_eq!(grid.get((2, 2)), 9);

        let data = "
        abc
        def
        ";
        let grid: Grid<char> = Grid::load(data, "");
        assert_eq!(grid.size(), (3, 2));

        let data = "
        123
        456
        789
        ";
        let grid: Grid<u8> = Grid::load(data, "");
        assert_eq!(grid.size(), (3, 3));
        let grid: Grid<u32> = Grid::load(data, ",");
        assert_eq!(grid.size(), (1, 3));

        let data = "
        1X3
        456
        ";
        let grid: Option<Grid<u8>> = Grid::parse(data, "");
        assert!(grid.is_none());
    }


    #[test]
    fn test_explore() {
        let data = "
        #####
        #...#
        #####
        ";
        let grid: Grid<char> = Grid::load(data, "");
        assert_eq!(grid.explore((2, 1), |_, _, _| true).count(), 15);
        assert_eq!(grid.explore((2, 1), |p, _, _| grid.get(p) != '#').count(), 3);
    }
}
