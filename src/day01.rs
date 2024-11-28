// Used to get the data
const DATA: &str = include_str!("../data/final/day01.txt");
pub fn part1(data: &str) -> u32 {
    let data = data.split('\n');
    let mut total = 0;
    for i in data {
        let mut first = None;
        let mut last = None;
        for j in i.chars() {
            let j = j.to_digit(10);
            if let Some(j) = j {
                if first.is_none() {
                    first = Some(j);
                }
                last = Some(j);
            }
        }
        total += first.unwrap_or(0) * 10 + last.unwrap_or(0);
    }
    total
}
const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
pub fn part2(data: &str) -> usize {
    let data = data.split('\n');
    let mut total = 0;
    for i in data {
        let mut first = None;
        let mut last = None;
        for j in 0..i.len() {
            let val = i.chars().nth(j).unwrap_or('a').to_digit(10);
            if val.is_some() {
                if first.is_none() {
                    first = Some(j);
                }
                last = Some(j);
            }
        }
        let mut num1 = i
            .chars()
            .nth(first.unwrap_or(0))
            .unwrap()
            .to_digit(10)
            .unwrap_or(0) as usize;
        let mut num2 = i
            .chars()
            .nth(last.unwrap_or(0))
            .unwrap()
            .to_digit(10)
            .unwrap_or(0) as usize;
        for (num, string) in NUMBERS.iter().enumerate() {
            let loc: Option<usize> = i
                .get(0..first.unwrap_or(string.len()))
                .unwrap_or("")
                .find(string);
            if let Some(loc) = loc {
                num1 = num + 1;
                first = Some(loc + string.len());
            }
            while let Some(loc) = i.get(last.unwrap_or(0)..).unwrap_or("").find(string) {
                num2 = num + 1;
                last = Some(last.unwrap_or(0) + loc + 1);
            }
        }
        total += num1 * 10 + num2;
    }
    total
}
pub fn main() {
    let answer = part1(DATA);
    println!("Answer for day 1 part 1 is {}.", answer);
    let answer = part2(DATA);
    println!("Answer for day 1 part 2 is {}.", answer);
}
