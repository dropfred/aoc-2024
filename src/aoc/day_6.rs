// #[repr(u8)]
// enum Direction {
//     N = b'^',
//     E = b'>',
//     S = b'v',
//     W = b'<'
// }

// enum Direction {
//     N,
//     E,
//     S,
//     W
// }

// impl Direction {
//     fn from_char(c: char) -> Self {
//         match c {
//             '^' => Direction::N,
//             '>' => Direction::E,
//             'v' => Direction::S,
//             '<' => Direction::W,
//             _ => panic!("invalid direction")
//         }
//     }
// }

#[derive(Eq, PartialEq)]
enum Step {
    New,
    Visited,
    Loop,
    Out
}

type Position = (i32, i32, char);

#[derive(Clone, Debug)]
struct Data {
    map: Vec<Vec<char>>,
    position: Option<Position>
}

impl Data {
    fn new(data: &str) -> Self {
        let map = data.trim().lines().map(|line| line.trim().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        // let size = (map[0].len() as i32, map.len() as i32);
        let mut position = None;
        for (y, we) in map.iter().enumerate() {
            if let Some(x) = we.iter().position(|c| "^v<>".contains(*c)) {
                position = Some((x as i32, y as i32, we[x]));
                break;
            }
        }
        Data {map, position}
    }

    fn next(&self) -> Option<Position> {
        self.position.and_then(|(x, y, dir)| {
            let (nx, ny) = match dir {
                '^' => (x, y - 1),
                '>' => (x + 1, y),
                'v' => (x, y + 1),
                '<' => (x - 1, y),
                _ => panic!("invalid direction")
            };
            let (sx, sy) = (self.map[0].len() as i32, self.map.len() as i32);
            if (nx >= 0) && (nx < sx) && (ny >= 0) && (ny < sy) {
                let c = self.map[ny as usize][nx as usize];
                if c == '#' {
                    let nd = match dir {
                        '^' => '>',
                        '>' => 'v',
                        'v' => '<',
                        '<' => '^',
                        _ => panic!("invalid direction")
                    };
                    Some((x, y, nd))
                } else {
                    Some((nx, ny, dir))
                }
            } else {
                None
            }
        })
    }

    fn step(&mut self) -> Step {
        let position = self.next();
        match position {
            Some((nx, ny, nd)) => {
                self.position = position;
                let c = &mut self.map[ny as usize][nx as usize];
                let s = if *c == '.' {*c = nd; Step::New} else if *c == nd {Step::Loop} else {Step::Visited};
                s
            },
            None => {
                self.position = None;
                Step::Out
            }
        }
    }

    // fn step(&mut self) -> Step {
    //     match self.position {
    //         Some((x, y, dir)) => {
    //             let (nx, ny) = match dir {
    //                 '^' => (x, y - 1),
    //                 '>' => (x + 1, y),
    //                 'v' => (x, y + 1),
    //                 '<' => (x - 1, y),
    //                 _ => panic!("invalid direction")
    //             };
    //             let (sx, sy) = (self.map[0].len() as i32, self.map.len() as i32);
    //             if (nx >= 0) && (nx < sx) && (ny >= 0) && (ny < sy) {
    //                 let c = &mut self.map[ny as usize][nx as usize];
    //                 if *c == '#' {
    //                     let nd = match dir {
    //                         '^' => '>',
    //                         '>' => 'v',
    //                         'v' => '<',
    //                         '<' => '^',
    //                         _ => panic!("invalid direction")
    //                     };
    //                     self.position = Some((x, y, nd));
    //                     Step::Visited
    //                 } else {
    //                     self.position = Some((nx, ny, dir));
    //                     let s = if *c == '.' {Step::New} else if *c == dir {Step::Loop} else {Step::Visited};
    //                     *c = dir;
    //                     s
    //                 }
    //             } else {
    //                 self.position = None;
    //                 Step::Out
    //             }
    //         },
    //         None => Step::Out
    //     }
    // }

    fn reset(&mut self, position: Option<Position>) {
        for cs in &mut self.map {
            for c in cs {
                if *c != '#' {
                    *c = '.';
                }
                self.position = position;
            }
        }
    }
}

fn part_1(data: &mut Data) -> u32 {
    let mut total = 1u32;
    while !data.position.is_none() {
        if data.step() == Step::New {total += 1;}
    }
    total
}

fn part_2(data: &mut Data) -> u32 {
    let mut total = 0;
    // while !data.position.is_none() {
    //     let test = data.clone();
    //     if data.step() == Step::New {total += 1;}
    // }
    total
}

pub fn solve() {
    let data = include_str!("../../data/day_6/input.txt");
    let mut data = Data::new(data);
    let start = data.position;
    println!("part 1: {}", part_1(&mut data));
    data.reset(start);
    println!("part 2: {}", part_2(&mut data));
}

mod tests {
    use super::*;

    const DATA: &str = include_str!("../../data/day_6/test.txt");

    #[test]
    fn test_data() {
        let data = Data::new(DATA);
        assert!(data.map.len() == 10);
        assert!(data.map.iter().all(|line| line.len() == 10));
    }

    #[test]
    fn test_part_1() {
        let mut data = Data::new(DATA);
        assert!(part_1(&mut data) == 41);
    }

    // #[test]
    // fn test_loop_1() {
    //     let mut data = Data::new("");
    // }
}