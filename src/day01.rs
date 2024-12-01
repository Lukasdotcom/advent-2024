use std::collections::{BinaryHeap, HashMap};
// Used to get the data
const DATA: &str = include_str!("../data/final/day01.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day01.txt");
    assert_eq!(calc(DATA_PART1).0, 11);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day01.txt");
    assert_eq!(calc(DATA_PART2).1, 31);
}
pub fn calc(data: &str) -> (u32, u32) {
    let mut first_half = BinaryHeap::new();
    let mut second_half = BinaryHeap::new();
    let mut second_half_amount = HashMap::<u32, u32>::default();
    for line in data.lines() {
        let mut nums = line
            .split(' ')
            .map(|s| s.parse::<u32>())
            .filter(|n| n.is_ok())
            .map(|n| n.unwrap());
        let next = nums.next();
        if next.is_some() {
            first_half.push(next.unwrap());
        }
        let next = nums.next();
        if next.is_some() {
            second_half.push(next.unwrap());
            *second_half_amount.entry(next.unwrap()).or_default() += 1;
        }
    }
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    while first_half.len() > 0 || second_half.len() > 0 {
        let first = first_half.pop().unwrap();
        let second = second_half.pop().unwrap();
        sum_part1 += first.abs_diff(second);
        sum_part2 += second_half_amount.get(&first).unwrap_or(&0) * first;
    }
    (sum_part1, sum_part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 1 part 1 is {}.", part1);
    println!("Answer for day 1 part 2 is {}.", part2);
}
