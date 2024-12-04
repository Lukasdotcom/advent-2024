// Used to get the data
const DATA: &str = include_str!("../data/final/day04.txt");
const SEARCH: &str = "XMAS";
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day04.txt");
    assert_eq!(part1(DATA_PART1), 18);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day04.txt");
    assert_eq!(part2(DATA_PART2), 9);
}

/// Checks if this cell forms a valid XMAS X
pub fn is_valid_part2(table: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if table[i][j] != 'A' {
        return false;
    }
    let empty = vec![];
    let top_left = table
        .get(i - 1)
        .unwrap_or(&empty)
        .get(j - 1)
        .unwrap_or(&' ');
    if top_left != &'M' && top_left != &'S' {
        return false;
    }
    let bottom_right = table
        .get(i + 1)
        .unwrap_or(&empty)
        .get(j + 1)
        .unwrap_or(&' ');
    if bottom_right != &'M' && bottom_right != &'S' {
        return false;
    }
    if top_left == bottom_right {
        return false;
    }
    let top_right = table
        .get(i - 1)
        .unwrap_or(&empty)
        .get(j + 1)
        .unwrap_or(&' ');
    if top_right != &'M' && top_right != &'S' {
        return false;
    }
    let bottom_left = table
        .get(i + 1)
        .unwrap_or(&empty)
        .get(j - 1)
        .unwrap_or(&' ');
    if bottom_left != &'M' && bottom_left != &'S' {
        return false;
    }
    if top_right == bottom_left {
        return false;
    }
    true
}
pub fn part2(data: &str) -> u32 {
    let mut count = 0;
    let table = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 1..table.len() {
        let row = &table[i];
        for j in 1..row.len() {
            if is_valid_part2(&table, i, j) {
                count += 1;
            }
        }
    }
    count
}
pub fn part1(data: &str) -> u32 {
    let mut count = 0;
    let table = data
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..table.len() {
        let row = &table[i];
        for j in 0..row.len() {
            if row[j] == SEARCH.chars().nth(0).unwrap() {
                'a: for direction in [
                    (-1, 0),
                    (1, 0),
                    (0, -1),
                    (0, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                    (1, 1),
                ] {
                    let mut coord = (i as i32, j as i32);
                    for char in SEARCH.chars().skip(1) {
                        coord.0 += direction.0;
                        coord.1 += direction.1;
                        if coord.0 < 0 || coord.0 >= table.len() as i32 {
                            continue 'a;
                        }
                        if coord.1 < 0 || coord.1 >= row.len() as i32 {
                            continue 'a;
                        }
                        if table[coord.0 as usize][coord.1 as usize] != char {
                            continue 'a;
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    count
}
pub fn main() {
    println!("Answer for day 4 part 1 is {}.", part1(DATA));
    println!("Answer for day 4 part 2 is {}.", part2(DATA));
}
