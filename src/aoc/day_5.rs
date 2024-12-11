use std::{cmp::Ordering, collections::HashMap};

struct Data {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>
}

impl Data {
    fn new(data: &str) -> Self {
        let data: Vec<&str> = data.lines().collect();
        let data = {
            let s = data.iter().position(|s| s.is_empty()).unwrap();
            (data[..s].iter(), data[(s + 1)..].iter())
        };

        let mut rules : HashMap<u32, Vec<u32>> = HashMap::new();
        for update in data.0 {
            let s = update.find('|').unwrap();
            let (p1, p2) = (update[..s].parse::<u32>().unwrap(), update[(s + 1)..].parse::<u32>().unwrap());
            rules.entry(p1).or_insert(Vec::new()).push(p2);
        }

        let updates: Vec<_> = data.1.map(|pages| {
            pages.split(',').map(|page| page.parse::<u32>().unwrap()).collect::<Vec<_>>()
        }).collect();
        // assert!(updates.iter().all(|u| (u.len() & 1) != 0));

        Data {rules, updates}
    }
}

fn is_valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, p) in update.iter().enumerate() {
        if rules.get(p).is_some_and(|rs| {rs.iter().any(|p| update[..i].contains(p))}) {
            return false;
        }
    }
    true
}

fn part_1(data: &Data) -> u32 {
    let mut total = 0;
    for update in &data.updates {
        if is_valid_update(update, &data.rules) {
            total += update[update.len() / 2];
        }
    }
    total
}


fn part_2(data: &Data) -> u32 {
    let mut total = 0;
    for update in &data.updates {
        if !is_valid_update(update, &data.rules) {
            let mut update = update.clone();
            update.sort_by(|p1, p2| {
                if      data.rules.get(p1).is_some_and(|rs| {rs.contains(p2)}) {Ordering::Less}
                else if data.rules.get(p2).is_some_and(|rs| {rs.contains(p1)}) {Ordering::Greater}
                else {Ordering::Equal}
            });
            total += update[update.len() / 2];
        }
    }
    total
}

pub fn solve() {
    let data = include_str!("../../data/day_5/input.txt"); // 6951 / 4121
    let data = Data::new(data);
    println!("part 1: {}", part_1(&data));
    println!("part 2: {}", part_2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        let data = include_str!("../../data/day_5/test.txt");
        let data = Data::new(data);
        assert!(data.updates.len() == 6);
        assert!(data.updates.iter().all(|u| (u.len() & 1) != 0));
        assert!(data.rules.len() == 6);
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../data/day_5/test.txt");
        let data = Data::new(data);
        assert!(part_1(&data) == 143);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../data/day_5/test.txt");
        let data = Data::new(data);
        assert!(part_2(&data) == 123);
    }
}
