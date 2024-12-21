use std::process::Output;

#[derive(Debug)]
struct Puzzle {
    ra: u64,
    rb: u64,
    rc: u64,
    program: Vec<u8>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let ra = data.next()?.trim().trim_start_matches("Register A: ").parse().ok()?;
        let rb = data.next()?.trim().trim_start_matches("Register B: ").parse().ok()?;
        let rc = data.next()?.trim().trim_start_matches("Register C: ").parse().ok()?;
        data.next();
        let program = data.next()?.trim().trim_start_matches("Program: ").split(",").map(|s| s.parse().unwrap()).collect();
        Some(Puzzle {ra, rb, rc, program})
    }

    fn combo(&self, op: u8) -> u64 {
        let op = op as u64;
        match op {
            0..=3 => op,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => panic!("invalid code")
        }
    }

    fn run(&mut self) -> String {
        let mut ip = 0;
        let mut output = String::new();
        while ip != self.program.len() {
            // println!("{ip} / {self:?}");
            let (opcode, operand) = (self.program[ip], self.program[ip + 1]);
            ip += 2;
            match opcode {
                0 => /* adv */ {
                    let n = self.ra;
                    let d = 1 << self.combo(operand);
                    self.ra = n / d;
                },
                1 => /* bxl */ {
                    self.rb ^= operand as u64;
                },
                2 => /* bst */ {
                    self.rb = self.combo(operand) & 7;
                },
                3 => /* jnz */ {
                    if self.ra != 0 {
                        ip = operand as usize;
                    }
                },
                4 => /* bxc */ {
                    self.rb ^= self.rc;
                },
                5 => /* out */ {
                    if !output.is_empty() {
                        output.push(',');
                    }
                    output.push_str((self.combo(operand) & 7).to_string().as_str());
                },
                6 => /* bdv */ {
                    let n = self.ra;
                    let d = 1 << self.combo(operand);
                    self.rb = n / d;
                },
                7 => /* cdv */ {
                    let n = self.ra;
                    let d = 1 << self.combo(operand);
                    self.rc = n / d;
                },
                _ => panic!("invalid opcode")
            }
        }
        // println!("{output}");
        output
    }
}

fn part_1() -> String {
    let puzzle = include_str!("../../data/day_17/input.txt");
    let mut puzzle = Puzzle::parse(puzzle).expect("invalid input");
    puzzle.run()
}

fn part_2() -> u64 {
    todo!("part 2");
}

pub fn solve() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let puzzle = include_str!("../../data/day_17/test_1.txt");
        let puzzle = Puzzle::parse(puzzle).unwrap();
        assert!(puzzle.ra == 729);
        assert!(puzzle.rb == 0);
        assert!(puzzle.rc == 0);
        assert!(puzzle.program.len() == 6);
    }

    #[test]
    fn test_run() {
        let mut puzzle = Puzzle {ra: 0, rb: 0, rc: 0, program: vec![5, 0]};
        assert!(puzzle.run() == "0");

        let mut puzzle = Puzzle {ra: 10, rb: 0, rc: 0, program: vec![5, 4]};
        assert!(puzzle.run() == "2");

        let mut puzzle = Puzzle {ra: 0, rb: 0, rc: 9, program: vec![2, 6]};
        assert!(puzzle.run() == "");
        assert!(puzzle.rb == 1);

        let mut puzzle = Puzzle {ra: 0, rb: 29, rc: 0, program: vec![1, 7]};
        assert!(puzzle.run() == "");
        assert!(puzzle.rb == 26);

        let mut puzzle = Puzzle {ra: 0, rb: 2024, rc: 43690, program: vec![4, 0]};
        assert!(puzzle.run() == "");
        assert!(puzzle.rb == 44354);

        let mut puzzle = Puzzle {ra: 10, rb: 0, rc: 0, program: vec![5,0,5,1,5,4]};
        assert!(puzzle.run() == "0,1,2");

        let mut puzzle = Puzzle {ra: 2024, rb: 0, rc: 0, program: vec![0,1,5,4,3,0]};
        assert!(puzzle.run() == "4,2,5,6,7,7,7,7,3,1,0");
        assert!(puzzle.ra == 0);

        let puzzle = include_str!("../../data/day_17/test_1.txt");
        let mut puzzle = Puzzle::parse(puzzle).unwrap();
        assert!(puzzle.run() == "4,6,3,5,6,3,5,2,1,0");
    }
}
