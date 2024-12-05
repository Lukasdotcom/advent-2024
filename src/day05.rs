use std::collections::{HashMap, HashSet};

// Used to get the data
const DATA: &str = include_str!("../data/final/day05.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day05.txt");
    assert_eq!(calc(DATA_PART1).0, 143);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day05.txt");
    assert_eq!(calc(DATA_PART2).1, 123);
}
pub fn valid_print(print: &Vec<u32>, requirements: &HashMap<u32, Vec<u32>>) -> bool {
    let mut set = HashSet::new();
    for print in print.iter() {
        if set.contains(print) {
            return false;
        }
        if let Some(reqs) = requirements.get(print) {
            for req in reqs {
                set.insert(req);
            }
        }
    }
    true
}
pub fn midpoint_for_correct_print(print: &Vec<u32>, requirements: &HashMap<u32, Vec<u32>>) -> u32 {
    let empty = vec![];
    for this_print in print.clone() {
        // Really cool trick found that says that if half of them are needed to be before this will be in the middle
        if requirements.get(&this_print).unwrap_or(&empty).iter().filter(|x| print.contains(*x)).count() == print.len() / 2 {
            return this_print;
        }
    }
    panic!("No midpoint found");
}
pub fn calc(data: &str) -> (u32, u32) {
    let mut part1_sum = 0;
    let mut part2_sum = 0;
    let (order, prints) = data.split_once("\n\n").unwrap();
    let mut requirements: HashMap<u32, Vec<u32>> = HashMap::new(); // Stores all the things that need to be printed before a certain print
    order.lines().for_each(|line| {
        let (before, after) = line.split_once("|").unwrap();
        let before = before.parse().unwrap();
        let after = after.parse().unwrap();
        if requirements.contains_key(&after) {
            requirements.get_mut(&after).unwrap().push(before);
        } else {
            requirements.insert(after, vec![before]);
        }
    });
    for print in prints.lines() {
        let print = print
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        // Adds middle element if valid
        if valid_print(&print, &requirements) {
            part1_sum += print[print.len() / 2];
        } else {
            part2_sum += midpoint_for_correct_print(&print, &requirements);
        }
    }
    (part1_sum, part2_sum)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 4 part 1 is {}.", part1);
    println!("Answer for day 4 part 2 is {}.", part2);
}
