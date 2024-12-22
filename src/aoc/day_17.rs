#[derive(Debug)]
struct Computer {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
    out: String
}

impl Computer {
    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let a = data.next()?.trim().trim_start_matches("Register A: ").parse().ok()?;
        let b = data.next()?.trim().trim_start_matches("Register B: ").parse().ok()?;
        let c = data.next()?.trim().trim_start_matches("Register C: ").parse().ok()?;
        data.next();
        let program = data.next()?.trim().trim_start_matches("Program: ").split(",").map(|s| s.parse().unwrap()).collect();
        // let program: Vec<_> = data.next()?.trim().trim_start_matches("Program: ").split(",").map(|s| s.parse().unwrap_or(255)).collect();
        // if program.iter().any(|c| *c == 255) {
        //     return None;
        // }
        Some(Computer::new(a, b, c, program))
    }

    fn new(a: u64, b: u64, c: u64, program: Vec<u8>) -> Self {
        Computer {a, b, c, program, ip: 0, out: String::new()}
    }

    // fn reset(&mut self, a: u64, b: u64, c: u64) {
    //     self.a = a;
    //     self.b = b;
    //     self.c = c;
    //     self.ip = 0;
    //     self.out = String::new();
    // }

    fn combo(&self, op: u8) -> u64 {
        let op = op as u64;
        match op {
            0..=3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo")
        }
    }

    fn adv(&mut self, operand: u8) {
        let n = self.a;
        let d = 1 << self.combo(operand);
        self.a = n / d;
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo(operand) & 7;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.ip = operand as usize;
        }
    }

    fn bxc(&mut self, _: u8) {
        self.b ^= self.c;
    }
    
    fn out(&mut self, operand: u8) {
        if !self.out.is_empty() {
            self.out.push(',');
        }
        self.out.push(((self.combo(operand) & 7) as u8 + b'0') as char);
    }

    fn bdv(&mut self, operand: u8) {
        let n = self.a;
        let d = 1 << self.combo(operand);
        self.b = n / d;
    }

    fn cdv(&mut self, operand: u8) {
        let n = self.a;
        let d = 1 << self.combo(operand);
        self.c = n / d;
    }

    fn run(&mut self) -> &str {
        while self.ip != self.program.len() {
            let (opcode, operand) = (self.program[self.ip], self.program[self.ip + 1]);
            self.ip += 2;
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("invalid opcode")
            }
        }

        self.out.as_str()
    }
}

impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let opcode = |c| match c {
            0 => "adv",
            1 => "bxl",
            2 => "bst",
            3 => "jnz",
            4 => "bxc",
            5 => "out",
            6 => "bdv",
            7 => "cdv",
            _ => panic!("invalid opcode")
        };
        writeln!(f, "A: {:X}", self.a)?;
        writeln!(f, "B: {:X}", self.b)?;
        writeln!(f, "C: {:X}", self.c)?;
        writeln!(f, "IP: {:X}", self.ip)?;
        writeln!(f, "")?;
        for i in 0..(self.program.len() / 2) {
            writeln!(f, "{} {}", opcode(self.program[i * 2]), self.program[i * 2 + 1])?;
        }
        writeln!(f, "")?;
        writeln!(f, "OUT: '{}'", self.out)?;
        Ok(())
    }
}

fn part_1() -> String {
    let puzzle = include_str!("../../data/day_17/input.txt");
    let mut puzzle = Computer::parse(puzzle).expect("valid input");
    println!("{}", puzzle);
    puzzle.run().to_string()
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
        let puzzle = Computer::parse(puzzle).unwrap();
        assert!(puzzle.a == 729);
        assert!(puzzle.b == 0);
        assert!(puzzle.c == 0);
        assert!(puzzle.program.len() == 6);
    }

    #[test]
    fn test_run() {
        let mut puzzle = Computer::new(0, 0, 0, vec![5, 0]);
        assert!(puzzle.run() == "0");

        let mut puzzle = Computer::new(10, 0, 0, vec![5, 4]);
        assert!(puzzle.run() == "2");

        let mut puzzle = Computer::new(0, 0, 9, vec![2, 6]);
        assert!(puzzle.run() == "");
        assert!(puzzle.b == 1);

        let mut puzzle = Computer::new(0, 29, 0, vec![1, 7]);
        assert!(puzzle.run() == "");
        assert!(puzzle.b == 26);

        let mut puzzle = Computer::new(0, 2024, 43690, vec![4, 0]);
        assert!(puzzle.run() == "");
        assert!(puzzle.b == 44354);

        let mut puzzle = Computer::new(10, 0, 0, vec![5,0,5,1,5,4]);
        assert!(puzzle.run() == "0,1,2");

        let mut puzzle = Computer::new(2024, 0, 0, vec![0,1,5,4,3,0]);
        assert!(puzzle.run() == "4,2,5,6,7,7,7,7,3,1,0");
        assert!(puzzle.a == 0);

        let puzzle = include_str!("../../data/day_17/test_1.txt");
        let mut puzzle = Computer::parse(puzzle).unwrap();
        assert!(puzzle.run() == "4,6,3,5,6,3,5,2,1,0");
    }
}
