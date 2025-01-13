struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32
}
struct Puzzle {
    robots: Vec<Robot>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_robot = |s: &str| {
            let (p, v) = s.split_once(" ")?;
            let (px, py) = p.trim_start_matches("p=").split_once(",")?;
            let (vx, vy) = v.trim_start_matches("v=").split_once(",")?;
            let (px, py) = (px.parse().ok()?, py.parse().ok()?);
            let (vx, vy) = (vx.parse().ok()?, vy.parse().ok()?);
            Some(Robot {px, py, vx, vy})
        };
        let robots: Option<_> = data.trim().lines().map(parse_robot).collect();
        let robots = robots?;
        Some(Puzzle {robots})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn quadrants(puzzle: &Puzzle, size: (i32, i32), time: i32) -> u32 {
    let (w, h) = size;
    let (w2, h2) = (w / 2, h / 2);
    let qs = puzzle.robots.iter().map(|r| {
        let (mut x, mut y) = ((r.px + r.vx * time) % w, (r.py + r.vy * time) % h);
        if x < 0 {x += w;}
        if y < 0 {y += h;}
        (x, y)
    }).filter(|(x, y)| {
        (*x != w2) && (*y != h2)
    }).map(|(x, y)| {
        (if x < w2 {0} else {1}, if y < h2 {0} else {1})
    }).fold((0, 0, 0, 0), |(q00, q01, q10, q11), q| {
        match q {
            (0, 0) => (q00 + 1, q01, q10, q11),
            (0, 1) => (q00, q01 + 1, q10, q11),
            (1, 0) => (q00, q01, q10 + 1, q11),
            (1, 1) => (q00, q01, q10, q11 + 1),
            _ => panic!("invalid quadrant")
        }
    });
    qs.0 * qs.1 * qs.2 * qs.3
}

fn part_1(puzzle: &Puzzle) -> u32 {
    quadrants(puzzle, (101, 103), 100)
}

fn part_2(puzzle: &Puzzle, size: (i32, i32)) -> u32 {
    let n = puzzle.robots.len();
    let (w, h) = size;
    let (w4, h4) = (w / 4, h / 4);
    // assume that most of the robots are in the middle of the map
    for t in 0..(w * h) {
        if puzzle.robots.iter().map(|r| {
            let (mut x, mut y) = ((r.px + r.vx * t) % w, (r.py + r.vy * t) % h);
            if x < 0 {x += w;}
            if y < 0 {y += h;}
            (x, y)
        }).filter(|(x, y)| {
            (*x > w4) && (*x < (w - w4)) && (*y > h4) && (*y < (h - h4))
        }).count() > (n / 2) {
            let mut map = Vec::new();
            for _ in 0..h {
                map.push(vec![' '; w as usize]);
            }
            for (x, y) in puzzle.robots.iter().map(|r| {
                let (mut x, mut y) = ((r.px + r.vx * t) % w, (r.py + r.vy * t) % h);
                if x < 0 {x += w;}
                if y < 0 {y += h;}
                (x, y)
            }) {
                map[y as usize][x as usize] = '#';
            }
            println!("time={t}");
            for s in map {
                println!("{}", s.iter().collect::<String>());
            }
            return t as u32;
        }
    }
    0
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_14/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle, (101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_14/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(puzzle.robots.len(), 12)
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_14/test.txt");
        let puzzle = Puzzle::load(data);
        assert_eq!(quadrants(&puzzle, (11, 7), 100), 12);
    }
}
