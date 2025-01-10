struct Equation {
    result: u64,
    values: Vec<u64>
}

struct Puzzle {
    equations: Vec<Equation>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_equation = |s: &str| {
            let (result, values) = s.trim().split_once(": ")?;
            let result = result.parse().ok()?;
            let values: Option<Vec<_>> = values.split(' ').map(|v| v.parse().ok()).collect();
            let values = values?;
            Some(Equation {result, values})
        };
        let equations: Option<Vec<_>> = data.trim().lines()
            .map(parse_equation).collect();
        let equations = equations?;
        Some(Puzzle {equations})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(data: &Puzzle) -> u64 {
    let mut total = 0u64;
    for test in &data.equations {
        let ops = 1u64 << (test.values.len() - 1);
        for ops in 0..ops {
            let mut t = test.values[0];
            for (i, v) in (&test.values[1..]).iter().enumerate() {
                if (ops & (1u64 << i)) == 0 {
                    t += v;
                } else {
                    t *= v;
                }
                if t > test.result {break;}
            }
            if t == test.result {
                total += test.result;
                break;
            }
        }
    }
    total
}

// use itertools::Itertools;

// enum Op {
//     Add,
//     Mul,
//     Concat
// }

// fn part_2(data: &Puzzle) -> u64 {
//     let mut total = 0u64;
//     for test in &data.equations {
//         let ops = test.values.len() - 1;
//         for ops in (0..ops).map(|_| [Op::Add, Op::Mul, Op::Concat].iter()).multi_cartesian_product() {
//             let mut t = test.values[0];
//             for (i, v) in (&test.values[1..]).iter().enumerate() {
//                 match ops[i] {
//                     Op::Add => t += v,
//                     Op::Mul => t *= v,
//                     Op::Concat => t = (t * 10u64.pow(v.ilog10() + 1)) + v
//                 }
//                 if t > test.result {break;}
//             }
//             if t == test.result {
//                 total += test.result;
//                 break;
//             }
//         }
//     }
//     total
// }

use std::collections::HashSet;

fn part_2(data: &Puzzle) -> u64 {
    let mut total = 0u64;
    for e in &data.equations {
        let vs = e.values[1..].iter().fold(HashSet::from([e.values[0]]), |vs, v| {
            vs.into_iter().flat_map(|total| {
                if total <= e.result {
                    [
                        total + v,
                        total * v,
                        total * 10u64.pow(v.ilog10() + 1) + v
                    ]
                } else {
                    [u64::MAX, u64::MAX, u64::MAX]
                }
            }).collect()
        });
        if vs.contains(&e.result) {
            total += e.result
        }
    }
    total
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_7/input.txt");
    let data = Puzzle::load(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
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
