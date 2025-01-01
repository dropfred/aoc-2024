#[derive(Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
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

fn solve_game(game: &Game) -> Option<(i64, i64)> {
    let b = game.button_b.1 * game.button_a.0 - game.button_b.0 * game.button_a.1;
    if b != 0 {
        let b = (game.prize.1 * game.button_a.0 - game.button_a.1 * game.prize.0) / b;
        let a = (game.prize.0 - b * game.button_b.0) / game.button_a.0;
        if ((a * game.button_a.0 + b * game.button_b.0) == game.prize.0) && ((a * game.button_a.1 + b * game.button_b.1) == game.prize.1) {
            Some((a, b))
        } else {
            None
        }
    } else {
        None
    }
}

fn part_1(data: &Data) -> i64 {
    data.games.iter().map(|g| {
        if let Some((a, b)) = solve_game(g) {
            3 * a + b
        } else {
            0
        }
    }).sum()
}

fn part_2(data: &Data) -> i64 {
    data.games.iter().map(|g| {
        let g = Game {prize: (g.prize.0 + 10000000000000, g.prize.1 + 10000000000000), ..*g};
        if let Some((a, b)) = solve_game(&g) {
            3 * a + b
        } else {
            0
        }
    }).sum()
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
        assert_eq!(data.games.len(), 4);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_13/test.txt");
        let data = Data::new(data);
        assert_eq!(part_1(&data), 480);
    }
}
