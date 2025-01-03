struct Data {
    fs: Vec<u32>,
    files: Vec<(usize, usize)>,
    spaces: Vec<(usize, usize)>
}

impl Data {
    fn new(data: &str) -> Self {
        let mut fs = Vec::new();
        let mut files = Vec::new();
        let mut spaces = Vec::new();
        for (i, c) in data.trim().chars().enumerate() {
            let n = c.to_digit(10).unwrap();
            let id = if (i & 1) == 0 {(i / 2) as u32} else {u32::MAX};
            (if id != u32::MAX {&mut files} else {&mut spaces}).push((fs.len(), n as usize));
            fs.extend(std::iter::repeat(id).take(n as usize));
        }
        Data {fs, files, spaces}
    }
}

fn checksum(fs: &Vec<u32>) -> u64 {
    fs.iter().enumerate()
             .filter(|(_, id)| **id != u32::MAX)
             .map(|(i, id)| (i as u64) * (*id as u64))
             .sum()
}

fn part_1(data: &Data) -> u64 {
    if data.fs.is_empty() {return 0;}
    let mut fs = data.fs.clone();
    let mut b = 0;
    let mut e = fs.len() - 1;
    loop {
        while (b < e) && (fs[b] != u32::MAX) {b += 1;}
        while (e > b) && (fs[e] == u32::MAX) {e -= 1;}
        if b < e {
            fs[b] = fs[e];
            fs[e] = u32::MAX;
            b += 1;
            e -= 1;
        } else {
            break;
        }
    }
    checksum(&fs)
}

fn part_2(data: &Data) -> u64 {
    if data.fs.is_empty() {return 0;}
    let mut fs = data.fs.clone();
    let mut spaces = data.spaces.clone();
    for file in data.files.iter().rev() {
        if let Some(space) = spaces.iter_mut().find(|space| space.1 >= file.1) {
            if space.0 < file.0 {
                for i in 0..(file.1) {
                    fs[space.0 + i] = fs[file.0 + i];
                    fs[file.0 + i] = u32::MAX;
                }
                space.1 -= file.1;
                space.0 += file.1;
            }
        }
    }
    checksum(&fs)
}

pub fn solve() {
    let data = include_str!("../../data/day_9/input.txt");
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        let fs = data.fs.iter().map(|id| if *id != u32::MAX {((*id as u8) + b'0') as char} else {'.'}).collect::<String>();
        assert_eq!(fs, "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        assert_eq!(part_1(&data), 1928);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_9/test.txt");
        let data = Data::new(data);
        assert_eq!(part_2(&data), 2858);
    }
}
