use std::collections::HashMap;

use crate::aoc::{grid::Grid, maze::Maze};

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
    let mut distances = Grid::new(maze.get_map().size(), usize::MAX);
    for (p, _, d) in maze.explore(end, wall) {
        distances.set(p, d);
    }
    let distance = distances.get(begin);
    let mut cheats = HashMap::new();
    for (d, p) in maze.get_path(begin, end, wall).unwrap().enumerate() {
        for (cp, _, cd) in maze.get_map().explore(p, |_, _, _| true).skip(1) {
            if cd > 1 {break;}
            if maze.get_map().get(cp) != wall {continue;}
            if !cheats.contains_key(&cp) {
                let mut ms: usize = 0;
                for (cp, _, cd) in maze.get_map().explore(cp, |_, _, _| true).skip(1) {
                    if cd > 1 {break;}
                    let ed = distances.get(cp);
                    if ed != usize::MAX {
                        let d = d + cd + ed + 1;
                        if d < distance {
                            ms = std::cmp::max(ms, distance - d);
                        }
                    }
                }
                cheats.insert(cp, ms);
            }
        }
    }
    cheats.into_iter().map(|((x, y), s)| ((x as usize, y as usize), s))
                      .filter(|(_, s)| *s > 0)
                      .collect()
}

fn solve_part_1(puzzle: &Puzzle, min: usize) -> usize {
    let maze = &puzzle.maze;
    let map = maze.get_map();
    let begin = map.find('S').unwrap();
    let end = map.find('E').unwrap();
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
        let begin = maze.get_map().find('S').unwrap();
        let end = maze.get_map().find('E').unwrap();
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
}
