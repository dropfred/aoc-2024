struct Equation {
    result: u64,
    values: Vec<u64>
}

struct Data {
    equations: Vec<Equation>
}

impl Data {
    fn new(data: &str) -> Self {
        let mut equations: Vec<Equation> = Vec::new();
        for test in data.trim().lines() {
            let p = test.find(':').unwrap();
            let result: u64 = test[0..p].parse().unwrap();
            let mut values: Vec<u64> = Vec::new();
            for v in test[p+1..].trim().split(' ') {
                values.push(v.parse::<u64>().unwrap());
            }
            equations.push(Equation {result, values});
        }
        Data {equations}
    }
}

fn part_1(data: &Data) -> u64 {
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

// fn part_2(data: &Data) -> u64 {
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

fn part_2(data: &Data) -> u64 {
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

pub fn solve() {
    let data = include_str!("../../data/day_7/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_7/test.txt");

    #[test]
    fn test_part_1() {
        let data = Data::new(DATA);
        assert_eq!(part_1(&data), 3749);
    }

    #[test]
    fn test_part_2() {
        let data = Data::new(DATA);
        assert_eq!(part_2(&data), 11387);
    }
}
