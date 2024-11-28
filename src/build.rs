use reqwest;
use std::fs;
use std::thread;
pub const DAYS: usize = 1;
fn main() {
    let cookie = std::env::var("AOC_COOKIE").unwrap();
    let mut handles = vec![];
    fs::create_dir_all("./data/final").expect("Failed to create final directory");
    for day in 1..=DAYS {
        let url = format!("https://adventofcode.com/2023/day/{}/input", day);
        let file = format!("./data/final/day{:02}.txt", day);
        if !std::path::Path::new(&file).exists() {
            println!("Downloading data for day {}", day);
            let cookie = cookie.clone();
            let handle = thread::spawn(move || {
                let client = reqwest::blocking::Client::new();
                let res = client
                    .get(url)
                    .header("cookie", format!("session={}", cookie))
                    .send();
                match res {
                    Ok(res) => {
                        let text = res.text().unwrap();
                        fs::write(&file, text).unwrap();
                    }
                    Err(err) => {
                        panic!("Failed to download data: {}", err);
                    }
                }
            });
            handles.push(handle);
        }
    }
    for handle in handles {
        if let Err(err) = handle.join() {
            panic!("Thread panicked: {:?}", err);
        }
    }
}
