use std::collections::HashSet;

enum Op {
    Add,
    Mul,
    Concat
}

struct Equation {
    result: u64,
    numbers: Vec<u64>
}

impl Equation {
    fn is_valid<const NOPS: usize>(&self, ops: &[Op; NOPS]) -> bool {
        if self.numbers.is_empty() {
            return false;
        }
        let mut ts = HashSet::from([self.numbers[0]]);
        for n in self.numbers.iter().skip(1) {
            ts = ts.iter().flat_map(|t| {
                ops.iter()
                    .map(move |op| {
                        match op {
                            Op::Add => t + n,
                            Op::Mul => t * n,
                            Op::Concat => (t * 10u64.pow(n.ilog10() + 1)) + n
                        }
                    })
                    .filter(|t| *t <= self.result)
            }).collect();
            if ts.is_empty() {return false;}
        }
        ts.into_iter().filter(|t| *t == self.result).count() != 0
    }
}

struct Puzzle {
    equations: Vec<Equation>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_equation = |s: &str| {
            let (result, numbers) = s.trim().split_once(": ")?;
            let result = result.parse().ok()?;
            let numbers: Option<Vec<_>> = numbers.split(' ').map(|v| v.parse().ok()).collect();
            let numbers = numbers?;
            Some(Equation {result, numbers})
        };
        let equations: Option<Vec<_>> = data.trim().lines().map(parse_equation).collect();
        let equations = equations?;
        Some(Puzzle {equations})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.equations.iter()
        .filter(|e| e.is_valid(&[Op::Add, Op::Mul]))
        .map(|e| e.result)
        .sum()
}

fn part_2(puzzle: &Puzzle) -> u64 {
    puzzle.equations.iter()
        .filter(|e| e.is_valid(&[Op::Add, Op::Mul, Op::Concat]))
        .map(|e| e.result)
        .sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_7/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_7/test.txt");

    #[test]
    fn test_part_1() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_1(&data), 3749);
    }

    #[test]
    fn test_part_2() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_2(&data), 11387);
    }
}
