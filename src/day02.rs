// Used to get the data
const DATA: &str = include_str!("../data/final/day02.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day02.txt");
    assert_eq!(calc(DATA_PART1).0, 2);
    assert!(is_safe_part1(&[1, 3, 6, 7, 9]));
}
#[test]
fn test_part2() {
    const DATA_PART1: &str = include_str!("../data/test/day02.txt");
    assert!(is_safe_part2(&[1, 3, 2, 4, 5]));
    assert_eq!(calc(DATA_PART1).1, 4);
}
pub fn is_safe_part2(levels: &[u32]) -> bool {
    for i in 0..levels.len() {
        let mut levels_copy = levels.to_vec();
        levels_copy.remove(i);
        if is_safe_part1(&levels_copy) {
            return true;
        }
    }
    false
}
pub fn is_safe_part1(levels: &[u32]) -> bool {
    let mut increasing = false; // If it ever increases
    let mut decreasing = false; // If it ever decreases
    for pair in levels.windows(2) {
        if pair[0] > pair[1] {
            decreasing = true;
        } else if pair[0] < pair[1] {
            increasing = true;
        }
        if pair[0] == pair[1] {
            return false;
        }
        if pair[0].abs_diff(pair[1]) > 3 {
            return false;
        }
    }
    increasing ^ decreasing
}
pub fn calc(data: &str) -> (u32, u32) {
    let mut safe_count_part1 = 0;
    let mut safe_count_part2 = 0;
    for line in data.lines() {
        let levels: Vec<u32> = line.split(' ').map(|s| s.parse::<u32>().unwrap()).collect();
        if is_safe_part1(&levels) {
            safe_count_part1 += 1;
        } else if is_safe_part2(&levels) {
            safe_count_part2 += 1;
        }
    }
    (safe_count_part1, safe_count_part2 + safe_count_part1)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 2 part 1 is {}.", part1);
    println!("Answer for day 2 part 2 is {}.", part2);
}
