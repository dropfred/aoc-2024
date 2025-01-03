use std::collections::HashMap;

struct Puzzle {
    secrets: Vec<u64>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let secrets: Vec<_> = data.trim().lines().map(|s| s.parse::<u64>()).collect();
        if secrets.iter().all(|s| s.is_ok()) {
            let secrets = secrets.into_iter().map(|s| s.unwrap()).collect();
            Some(Puzzle {secrets})
        } else  {
            None
        }
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_next_secret(secret: u64) -> u64 {
    let mut s = secret;
    s = ((s * 64  ) ^ s) % 16777216;
    s = ((s / 32  ) ^ s) % 16777216;
    s = ((s * 2048) ^ s) % 16777216;
    s
}

fn get_nth_secret(secret: u64, nth: usize) -> u64 {
    let mut s = secret;
    for _ in 0..nth {
        s = get_next_secret(s);
    }
    s
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.secrets.iter().map(|s| get_nth_secret(*s, 2000)).sum()
}

fn part_2(puzzle: &Puzzle) -> u64 {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_22/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1:  {}", part_1(&puzzle));
    println!("part 2:  {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_secret() {
        let mut s = 123;
        s = get_next_secret(s); assert_eq!(s, 15887950);
        s = get_next_secret(s); assert_eq!(s, 16495136);
        s = get_next_secret(s); assert_eq!(s, 527345);
        s = get_next_secret(s); assert_eq!(s, 704524);
        s = get_next_secret(s); assert_eq!(s, 1553684);
        s = get_next_secret(s); assert_eq!(s, 12683156);
        s = get_next_secret(s); assert_eq!(s, 11100544);
        s = get_next_secret(s); assert_eq!(s, 12249484);
        s = get_next_secret(s); assert_eq!(s, 7753432);
        s = get_next_secret(s); assert_eq!(s, 5908254);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_22/test_1.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_1(&puzzle), 37327623);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_22/test_2.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(part_2(&puzzle), 27);
    }
}
