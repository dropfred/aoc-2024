use std::collections::{HashMap, HashSet};

struct Puzzle {
    connections: Vec<(u16, u16)>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_connection = |s: &str| {
            let (a, b) = s.trim().split_once("-")?;
            let (a, b) = (a.trim(), b.trim());
            if a.is_ascii() && (a.len() == 2) && b.is_ascii() && (b.len() == 2) {
                let (a, b) = (a.as_bytes(), b.as_bytes());
                let a= (a[1] as u16) | ((a[0] as u16) << 8);
                let b= (b[1] as u16) | ((b[0] as u16) << 8);
                if a < b {Some((a, b))} else {Some((b, a))}
            } else {
                None
            }
        };
        let connections: Option<Vec<_>> = data.trim().lines().map(parse_connection).collect();
        let connections = connections?;
        Some(Self {connections})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_trios(puzzle: &Puzzle) -> Vec<(u16, u16, u16)> {
    let mut connections: HashMap<_, _> = HashMap::new();
    for (a, b) in &puzzle.connections {
        let c = connections.entry(*a).or_insert(HashSet::new());
        c.insert(*b);
    }

    let mut ts = Vec::new();
    for (a, bs) in &connections {
        for b in bs {
            if let Some(cs) = connections.get(b) {
                for c in cs {
                    if bs.contains(c) {
                        ts.push((*a, *b, *c));
                    }
                }
            }
        }
    };
    ts.sort();
    ts
}

// fn print_computer(computer: u16) {
//     let (a, b) = (computer & 0xff, computer >> 8);
//     let (a, b) = (a as u8  as char, b as u8 as char);
//     print!("{b}{a}");
// }

fn part_1(puzzle: &Puzzle) -> usize {
    let ts = get_trios(puzzle);
    let ts = ts.into_iter().filter(|(a, b, c)| {
        let t = 't' as u8;
        (((a >> 8) as u8) == t) || (((b >> 8) as u8) == t)  || (((c >> 8) as u8) == t)
    }).count();

    ts
}

fn part_2(puzzle: &Puzzle) -> usize {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_23/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "ab-cd";
        let puzzle = Puzzle::parse(data);
        assert!(puzzle.is_some());

        let data = "ab=cd";
        let puzzle = Puzzle::parse(data);
        assert!(puzzle.is_none());

        let data = "ab-cd-ef";
        let puzzle = Puzzle::parse(data);
        assert!(puzzle.is_none());
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_23/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 7);
    }

    #[test]
    fn test_part_2() {
    }
}