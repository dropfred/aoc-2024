use std::collections::HashMap;

struct Data(Vec<u32>, Vec<u32>);

impl Data {
    fn new(data: &str) -> Self {
        let mut v0 : Vec<u32> = Vec::new();
        let mut v1 : Vec<u32> = Vec::new();
        for line in data.lines() {
            let mut vs = line.split_ascii_whitespace();
            v0.push(vs.next().unwrap().parse::<u32>().unwrap());
            v1.push(vs.next().unwrap().parse::<u32>().unwrap());
        }
        Data(v0, v1)
    }
}

fn part_1(data: &Data) -> u32 {
    let (mut v0, mut v1) = (data.0.clone(), data.1.clone());
    v0.sort();
    v1.sort();

    v0.iter().zip(v1.iter()).map(|(v1, v2)| if v1 > v2 {v1 - v2} else {v2 - v1}).sum()
}

fn part_2(data: &Data) -> u32 {
    let (v0, v1) = (&data.0, &data.1);

    let mut v2 : HashMap<u32, u32> = HashMap::new();
    for v in v1 {
        *v2.entry(*v).or_insert(0) += 1;
    }

    let mut distance = 0;
    for v in v0 {
        if v2.contains_key(v) {
            distance += v2[&v] * v;
        }
    }
    distance
}

pub fn solve() {
    let data = include_str!("../../data/day_1/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_1/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 11);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_1/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 31);
    }
}
