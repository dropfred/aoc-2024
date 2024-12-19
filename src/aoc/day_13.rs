#[derive(Debug)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64)
}
struct Data {
    games: Vec<Game>
}

impl Data {
    fn new(data: &str) -> Self {
        let games = data.trim().replace("\r", "").split("\n\n").map(|abp| {
            let mut abp = abp.lines();
            let (a_x, a_y) = abp.next().unwrap().trim().split_once(": ").unwrap().1.split_once(", ").unwrap();
            let (b_x, b_y) = abp.next().unwrap().trim().split_once(": ").unwrap().1.split_once(", ").unwrap();
            let (p_x, p_y) = abp.next().unwrap().trim().split_once(": ").unwrap().1.split_once(", ").unwrap();
            let button_a = (a_x.trim_start_matches("X+").parse().unwrap(), a_y.trim_start_matches("Y+").parse().unwrap());
            let button_b = (b_x.trim_start_matches("X+").parse().unwrap(), b_y.trim_start_matches("Y+").parse().unwrap());
            let prize = (p_x.trim_start_matches("X=").parse().unwrap(), p_y.trim_start_matches("Y=").parse().unwrap());
            Game {button_a, button_b, prize}
        }).collect();
        Data {games}
    }
}

fn part_1(data: &Data) -> u64 {
    let mut games = Vec::new();
    for g in &data.games {
        for a in 0..=(g.prize.0 / g.button_a.0) {
            let dax = a * g.button_a.0;
            let dbx = g.prize.0 - dax;
            if (dbx % g.button_b.0) == 0 {
                let b = dbx / g.button_b.0;
                if (a * g.button_a.1 + b * g.button_b.1) == g.prize.1 {
                    games.push(a * 3 + b);
                    break;
                }
            }
        }
    }
    games.iter().sum()
}

fn part_2(data: &Data) -> u64 {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_13/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_13/test.txt");
        let data = Data::new(data);
        assert!(data.games.len() == 4);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_13/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 480);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_13/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 0);
    }
}
