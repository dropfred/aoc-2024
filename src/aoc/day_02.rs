struct Puzzle {
    reports: Vec<Vec<u32>>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_nums = |s: &str| {
            s.trim().split_ascii_whitespace().map(|s| s.parse()).collect()
        };
        let reports: Result<_, _> = data.lines().map(parse_nums).collect();
        let reports = reports.ok()?;
        Some(Puzzle {reports})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn is_safe(levels: &Vec<u32>) -> bool {
    if levels.len() < 2 {
        return true;
    }
    let inc = (levels[1] as i32 - levels[0] as i32) > 0;
    if levels.windows(2).any(|ls| {
        let d = ls[1] as i32 - ls[0] as i32;
        (d == 0) || (d.abs() > 3) || ((d > 0) != inc)
    }) {
        return false;
    }
    true
}

fn part_1(data: &Puzzle) -> usize {
    data.reports.iter().filter(|ls| is_safe(ls)).count()
}

fn part_2(data: &Puzzle) -> usize {
    data.reports.iter().filter(|ls| {
        is_safe(ls) || {
            (0..ls.len()).any(|r| {
                let vs = (0..r).chain((r + 1)..ls.len()).map(|i| ls[i]).collect();
                is_safe(&vs)
            })
        }
    }).count()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_02/input.txt");
    let data = Puzzle::load(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_02/test.txt");

    #[test]
    fn test_data() {
        let data = Puzzle::load(DATA);
        assert_eq!(data.reports.len(), 6);
        assert!(data.reports.iter().all(|levels| levels.len() == 5));
    }

    #[test]
    fn test_part_1() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_1(&data), 2);
    }

    #[test]
    fn test_part_2() {
        let data = Puzzle::load(DATA);
        assert_eq!(part_2(&data), 4);
    }
}
