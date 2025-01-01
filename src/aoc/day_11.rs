use std::collections::HashMap;

struct Data {
    stones: Vec<u64>
}

impl Data {
    fn new(data: &str) -> Self {
        let stones = data.trim().split(' ').map(|s| s.parse().unwrap()).collect();
        Data {stones}
    }

    fn blink(&self, blinks: u8) -> u64 {
        let mut ss = self.stones.iter().fold(HashMap::new(), |mut m, v| {*m.entry(*v).or_insert(0) += 1; m});
        for _ in 0..blinks {
            let mut nss = HashMap::new();
            for (v, n) in ss {
                if v == 0 {
                    *nss.entry(1).or_insert(0) += n;
                } else {
                    let ss = v.ilog10() + 1;
                    if (ss & 1) == 0 {
                        let p = 10u64.pow(ss / 2);
                        *nss.entry(v / p).or_insert(0) += n;
                        *nss.entry(v % p).or_insert(0) += n;
                    } else {
                        *nss.entry(v * 2024).or_insert(0) += n;
                    }
                }
            }
            ss = nss;
        }
        ss.values().fold(0, |t, n| t + n)
    }
}

fn part_1(data: &Data) -> u64 {
    data.blink(25)
}

fn part_2(data: &Data) -> u64 {
    data.blink(75)
}

pub fn solve() {
    let data = include_str!("../../data/day_11/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_11/test.txt");
        let data = Data::new(data);
        assert_eq!(data.stones.len(), 2);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_11/test.txt");
        let data = Data::new(data);
        assert_eq!(part_1(&data), 55312);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_11/input.txt");
        let data = Data::new(data);
        assert_eq!(part_2(&data), 221632504974231);
    }
}
