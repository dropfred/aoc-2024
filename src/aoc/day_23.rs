struct Puzzle {
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        None
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> usize {
    todo!("part 1");
}

fn part_2(puzzle: &Puzzle) -> usize {
    todo!("part 2");
}

pub fn solve() {
    let puzzle = Puzzle {};
    part_1(&puzzle);
    part_2(&puzzle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "
        ab-cd
        ";
        let puzzle = Puzzle::parse(data);
        assert
    }

    #[test]
    fn test_part_1() {
    }

    #[test]
    fn test_part_2() {
    }
}