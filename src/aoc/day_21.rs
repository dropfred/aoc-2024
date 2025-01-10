use crate::aoc::{grid::Grid, maze::Maze};

/*
numeric keypad

+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

directional keypad

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

*/

const NUMERIC_KEYPAD: &str = "
789
456
123
_0A
";

const DIRECTIONAL_KEYPAD: &str = "
_^A
<v>
";

const CODES: &str = "
341A
083A
802A
973A
780A
";

/*
type Keypad = Grid<char>;

#[derive(Debug, Clone)]
struct Robot {
    pad: Keypad,
    position: (usize, usize)
}

struct Puzzle {
    robots: [Robot; 4]
}

impl Puzzle {
    fn load() -> Self {
        let robots = [
            NUMERIC_KEYPAD,
            DIRECTIONAL_KEYPAD,
            DIRECTIONAL_KEYPAD,
            DIRECTIONAL_KEYPAD
        ];
        let robots: Vec<_> = robots.into_iter().map(|d| {
            let pad = Keypad::load(d, "");
            let position = pad.find('A').unwrap();
            Robot {pad, position}
        }).collect();
        let robots = <[Robot; 4]>::try_from(robots).unwrap();
        Self {robots}
    }
}
*/

struct Puzzle;

type Keypad = Maze;

fn get_dirs(pad: &Keypad, begin: (usize, usize), end: (usize, usize)) -> Option<String> {
    let path: Vec<_> = pad.get_path(begin, end, '_')?.collect();
    println!("begin: {:?}, end: {:?}", begin, end);
    println!("path: {:?}", path);
    let mut dirs = String::new();
    for w in path.windows(2) {
        match w {
            &[p1, p2] => {
                let (x1, y1) = (p1.0 as i32, p1.1 as i32);
                let (x2, y2) = (p2.0 as i32, p2.1 as i32);
                let d = ((x2 - x1), (y2 - y1));
                match d {
                    (-1, 0) => dirs.push('<'),
                    (1, 0)  => dirs.push('>'),
                    (0, -1) => dirs.push('^'),
                    (0, 1)  => dirs.push('v'),
                    _ => panic!("invalid dir {:?}", d)
                }
            },
            _ => return None
        }
    }
    Some(dirs)
}

fn part_1(puzzle: &Puzzle) -> usize {
    // let mut robots = puzzle.robots.clone();
    let dpad_0 = Keypad::load(DIRECTIONAL_KEYPAD);
    let npad = Keypad::load(NUMERIC_KEYPAD);
    // println!("{:?}", get_dirs(&npad, npad.get_map().find('A').unwrap(), npad.get_map().find('4').unwrap()));
    println!("{:?}", get_dirs(&npad, npad.get_map().find('7').unwrap(), npad.get_map().find('0').unwrap()));
    
    todo!("part 1");
}

fn part_2(puzzle: &Puzzle) -> usize {
    todo!("part 2");
}

pub(crate) fn solve() {
    // let puzzle = Puzzle::load();
    let puzzle = Puzzle {};
    part_1(&puzzle);
    part_2(&puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}