struct Data {
    reports: Vec<Vec<u32>>
}

impl Data {
    fn new(data: &str) -> Self {
        let mut reports = Vec::new();
        for line in data.lines() {
            reports.push(line.split_ascii_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        }
        Data {reports}
    }
}

fn is_safe(levels: &Vec<u32>) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let mut inc = None;
    for i in 1..levels.len() {
        let lvl0 = levels[i - 1] as i32;
        let lvl1 = levels[i] as i32;
        let d = lvl1 - lvl0;
        if (d == 0) || (d.abs() > 3) {
            return false;
        }
        match inc {
            Some(b) => {
                if (d > 0) != b {
                    return false;
                }
            },
            None => {
                inc = Some(d > 0);
            }
        }
    }
    true
}

fn part_1(data: &Data) -> u32 {
    let mut ss = 0;
    for levels in &data.reports {
        if is_safe(levels) {
            ss += 1;
        }
    }
    ss
}

fn part_2(data: &Data) -> u32 {
    let mut ss = 0;
    for levels in &data.reports {
        if is_safe(levels) {
            ss += 1;
        } else {
            for i in 0..levels.len() {
                let mut vs = levels.clone();
                vs.remove(i);
                if is_safe(&vs) {
                    ss += 1;
                    break;
                }
            }
        }
    }
    ss
}

pub fn solve() {
    let data = include_str!("../../data/day_2/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_2/test.txt");

    #[test]
    fn test_data() {
        let data = Data::new(DATA);
        assert_eq!(data.reports.len(), 6);
        assert!(data.reports.iter().all(|levels| levels.len() == 5));
    }

    #[test]
    fn test_part_1() {
        let data = Data::new(DATA);
        assert_eq!(part_1(&data), 2);
    }

    #[test]
    fn test_part_2() {
        let data = Data::new(DATA);
        assert_eq!(part_2(&data), 4);
    }
}
