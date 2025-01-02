use crate::aoc::grid::{Grid, GridExploreIterator};

pub type MazeExploreIterator<'a, S>  = GridExploreIterator<'a, char, S>;

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

    pub fn explore<S: FnMut((usize, usize), char) -> bool>(&self, start: (usize, usize), step: S) -> MazeExploreIterator<S> {
        MazeExploreIterator::new(&self.map, start, step)
    }

    pub fn get_distance_between(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> Option<usize> {
        for (p, d) in self.map.explore(begin, |_, c| {
            c != wall
        }) {
            if p == end {
                return Some(d)
            }
        }
        None
    }

    pub fn get_path_between(&self, begin: (usize, usize), end: (usize, usize), wall: char) -> Option<Vec<(usize, usize)>> {
        let mut distances = Grid::new(self.map.size(), usize::MAX);
        for (p, d) in self.explore(end, |_, c| c != wall) {
            distances.set(p, d);
        }
        if distances.get(begin) != usize::MAX {
            let mut path = Vec::new();
            let mut i = 0;
            for (p, d) in self.explore(begin, |_, c| c != wall) {
                if d == i {
                    path.push(p);
                    if p == end {
                        return Some(path)
                    }
                    i += 1;
                }
            }
        }
        None
    }

    pub fn get_map(&self) -> &Grid<char> {
        &self.map
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
        ###";
        let maze = Maze::load(data);
        assert_eq!(maze.map.size(), (3, 3));
    }

    #[test]
    fn test_distance_between() {
        let data = "#";
        let maze = Maze::load(data);
        assert_eq!(maze.get_distance_between((0, 0), (0, 0), '#'), None);

        let data = "
        #####
        #B#E#
        #####";
        let maze = Maze::load(data);
        let begin = maze.map.find('B').unwrap();
        let end = maze.map.find('E').unwrap();
        assert_eq!(maze.get_distance_between(begin, end, '#'), None);

        let data = "
        ###
        #.#
        ###";
        let maze = Maze::load(data);
        assert_eq!(maze.get_distance_between((1, 1), (1, 1), '#'), Some(0));

        let data = "
        ####
        #BE#
        ####";
        let maze = Maze::load(data);
        let begin = maze.map.find('B').unwrap();
        let end = maze.map.find('E').unwrap();
        assert_eq!(maze.get_distance_between(begin, end, '#'), Some(1));

        let data = "
        #####
        #B#E#
        #...#
        #####";
        let maze = Maze::load(data);
        let begin = maze.map.find('B').unwrap();
        let end = maze.map.find('E').unwrap();
        assert_eq!(maze.get_distance_between(begin, end, '#'), Some(4));

        let data = "
        #########
        #B....#.#
        #####...#
        #E......#
        #########
        ";
        let maze = Maze::load(data);
        let begin = maze.map.find('B').unwrap();
        let end = maze.map.find('E').unwrap();
        assert_eq!(maze.get_distance_between(begin, end, '#'), Some(10));
    }
}
