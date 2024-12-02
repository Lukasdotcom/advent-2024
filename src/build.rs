use reqwest;
use std::fs;
use std::thread;
pub const DAYS: usize = 2;
fn main() {
    let cookie = std::env::var("AOC_COOKIE");
    let mut handles = vec![];
    fs::create_dir_all("./data/final").expect("Failed to create final directory");
    for day in 1..=DAYS {
        let url = format!("https://adventofcode.com/2024/day/{}/input", day);
        let file = format!("./data/final/day{:02}.txt", day);
        if !std::path::Path::new(&file).exists() {
            println!("Downloading data for day {}", day);
            let cookie = cookie.clone();
            let handle = thread::spawn(move || match cookie {
                Ok(cookie) => {
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
                }
                Err(_) => {
                    fs::write(&file, "").unwrap();
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
