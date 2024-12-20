struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32
}
struct Data {
    robots: Vec<Robot>
}

impl Data {
    fn parse(data: &str) -> Self {
        let robots = data.trim().lines().map(|s| {
            let (p, v) = s.split_once(" ").unwrap();
            let (px, py) = p.trim_start_matches("p=").split_once(",").unwrap();
            let (vx, vy) = v.trim_start_matches("v=").split_once(",").unwrap();
            let (px, py) = (px.parse().unwrap(), py.parse().unwrap());
            let (vx, vy) = (vx.parse().unwrap(), vy.parse().unwrap());
            Robot {px, py, vx, vy}
        }).collect();
        Data {robots}
    }
}

fn quadrants(data: &Data, size: (i32, i32), time: i32) -> u32 {
    let (w, h) = size;
    let (w2, h2) = (w / 2, h / 2);
    let qs = data.robots.iter().map(|r| {
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

fn part_1(data: &Data) -> u32 {
    quadrants(data, (101, 103), 100)
}

fn part_2(data: &Data, size: (i32, i32)) -> u32 {
    let n = data.robots.len();
    let (w, h) = size;
    let (w4, h4) = (w / 4, h / 4);
    // assume that most of the robots are in the middle of the map
    for t in 0..(w * h) {
        if data.robots.iter().map(|r| {
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
            for (x, y) in data.robots.iter().map(|r| {
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

pub fn solve() {
    let data = include_str!("../../data/day_14/input.txt");
    let data = Data::parse(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data, (101, 103)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_14/test.txt");
        let data = Data::parse(data);
        assert!(data.robots.len() == 12)
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_14/test.txt");
        let data = Data::parse(data);
        assert!(quadrants(&data, (11, 7), 100) == 12);
    }
}
