use reqwest;
use std::fs;
use std::thread;
pub const DAYS: usize = 3;
fn get_cookie() -> Result<String, String> {
    let file = "./cookie";
    if std::path::Path::new(file).exists() {
        let contents = fs::read_to_string(file);
        match contents {
            Ok(contents) => Ok(contents.trim().to_string()),
            Err(err) => Err(format!("Failed to read cookie file: {}", err)),
        }
    } else {
        Err("Cookie file does not exist".into())
    }
}
fn download_file(day: usize, file: &str, cookie: String) {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("cookie", format!("session={}", cookie))
        .send();
    println!("Downloading data for day {}", day);
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
fn main() {
    println!("cargo:rerun-if-changed=./data/final");
    let cookie = get_cookie();
    let mut handles = vec![];
    fs::create_dir_all("./data/final").expect("Failed to create final directory");
    for day in 1..=DAYS {
        let file = format!("./data/final/day{:02}.txt", day);
        if !std::path::Path::new(&file).exists() {
            fs::write(&file, "").unwrap();
        }
        if let Ok(cookie) = cookie.clone() {
            if let Ok(metadata) = fs::metadata(&file) {
                if metadata.len() == 0 {
                    handles.push(thread::spawn(move || download_file(day, &file, cookie)));
                }
            }
        }
    }
    for handle in handles {
        if let Err(err) = handle.join() {
            panic!("Thread panicked: {:?}", err);
        }
    }
}
