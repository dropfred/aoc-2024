use crate::aoc::grid::{Grid, GridExploreIterator};

pub type MazeExploreIterator<'a, F>  = GridExploreIterator<'a, char, F>;

pub struct MazePathIterator {
    path: Vec<(usize, usize)>
}

impl Iterator for MazePathIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.path.pop()
    }
}

pub struct Maze {
    map: Grid<char>
}

impl Maze {
    pub fn parse(data: &str) -> Option<Self> {
        Some(Self {map: Grid::parse(data, "")?})
    }

    pub fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    pub fn get_map(&self) -> &Grid<char> {
        &self.map
    }

    pub fn explore(&self, start: (usize, usize), wall: char) -> MazeExploreIterator<impl FnMut((usize, usize)) -> bool + '_> {
        MazeExploreIterator::new(self.get_map(), start, move |p| self.get_map().get(p) != wall)
    }

    pub fn get_path(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> MazePathIterator {
        let mut distances = Grid::new(self.map.size(), usize::MAX);
        for (p, d) in self.explore(begin, wall) {
            distances.set(p, d);
        }
        let mut path = Vec::new();
        if distances.get(end) != usize::MAX {
            let mut i = 0;
            for (p, d) in self.explore(end, wall) {
                if d == i {
                    path.push(p);
                    if p == begin {
                        break;
                    }
                    i += 1;
                }
            }
        }
        MazePathIterator {path}
    }

    pub fn get_distance(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> Option<usize> {
        for (p, d) in self.explore(begin, wall) {
            if p == end {
                return Some(d)
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "
        ###
        #.#
        ###
        ";
        let maze = Maze::load(data);
        assert_eq!(maze.map.size(), (3, 3));
    }

    #[test]
    fn test_explore() {
        let data = "
        #####
        #...#
        #####
        ";
        let maze = Maze::load(data);
        assert_eq!(maze.explore((2, 1), '#').count(), 3);
        assert_eq!(maze.explore((2, 1), '.').count(), 0);
    }

    #[test]
    fn test_get_distance() {
        let data = "
        #####
        #B#E#
        #...#
        #####
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), '#').unwrap(), 4);
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), 'X').unwrap(), 2);

        let data = "
        #######
        #B###E#
        #.#.#.#
        #.....#
        #######
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        assert_eq!(maze.get_distance(map.find('B').unwrap(), map.find('E').unwrap(), '#').unwrap(), 8);

        let data = "
        #########
        #B....#.#
        #####...#
        #E......#
        #########
        ";
        let maze = Maze::load(data);
        let begin = maze.get_map().find('B').unwrap();
        let end = maze.get_map().find('E').unwrap();
        assert_eq!(maze.get_distance(begin, end, '#'), Some(10));
    }

    #[test]
    fn test_get_path() {
        let data = "
        ####
        #BE#
        ####
        ";
        let maze = Maze::load(data);
        let map = &maze.get_map();
        let mut path = maze.get_path(map.find('B').unwrap(), map.find('E').unwrap(), '#');
        assert_eq!(path.next(), Some((1, 1)));
        assert_eq!(path.next(), Some((2, 1)));
        assert_eq!(path.next(), None);
    }
}
