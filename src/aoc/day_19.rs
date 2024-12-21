#[derive(Debug)]
struct Data {
    patterns: Vec<String>,
    designs: Vec<String>
}

impl Data {
    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let patterns = data.next()?.split(", ").map(String::from).collect();
        data.next();
        let designs = data.map(String::from).collect();
        Some(Data {patterns, designs})
    }
}

fn part_1(data: &Data) -> u32 {
    let mut c = 0;
    for d in &data.designs {
        let mut ds = Vec::new();
        ds.push(0);
        while !ds.is_empty() {
            let m = ds.pop().unwrap();
            if m == d.len() {
                c += 1;
                break;
            }
            let s = &d[m..];
            for p in &data.patterns {
                if s.starts_with(p) {
                    ds.push(m + p.len());
                }
            }
        }
    }
    c
}

fn part_2(data: &Data) -> u32 {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_19/input.txt");
    let data = Data::parse(data).expect("bad input");
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert!(data.patterns.len() == 8);
        assert!(data.designs.len() == 8);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert!(part_1(&data) == 6);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_19/test.txt");
        let data = Data::parse(data).unwrap();
        assert!(part_2(&data) == 16);
    }
}
