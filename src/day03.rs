use regex::Regex;
// Used to get the data
const DATA: &str = include_str!("../data/final/day03.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day03_part1.txt");
    assert_eq!(calc(DATA_PART1).0, 161);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day03_part2.txt");
    assert_eq!(calc(DATA_PART2).1, 48);
}
pub fn calc(data: &str) -> (u32, u32) {
    let pattern = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;
    let mut sum_part2 = 0;
    let mut enabled = true;
    for caps in re.captures_iter(data) {
        if caps.get(0).unwrap().as_str() == "do()" {
            enabled = true;
            continue;
        }
        if caps.get(0).unwrap().as_str() == "don't()" {
            enabled = false;
            continue;
        }
        let a = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let b = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        sum += a * b;
        if enabled {
            sum_part2 += a * b;
        }
    }
    (sum, sum_part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 3 part 1 is {}.", part1);
    println!("Answer for day 3 part 2 is {}.", part2);
}
