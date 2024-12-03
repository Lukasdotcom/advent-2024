use std::time::{Duration, Instant};
// The day to show (0 means all of them)
const DAY: u32 = 0;
mod day01;
mod day02;
mod day03;

fn main() {
    let tasks: [fn(); 3] = [day01::main, day02::main, day03::main];
    let now = Instant::now();
    if DAY == 0 {
        for task in tasks {
            task();
        }
        print_data("All the days", now.elapsed());
    } else {
        #[allow(arithmetic_overflow)]
        tasks[DAY as usize - 1]();
        print_data(&format!("Day {DAY}"), now.elapsed());
    }
}
fn print_data(data: &str, time: Duration) {
    let time = time.as_nanos() as f32;
    let digits = (time).log10().ceil() as usize;
    match digits {
        0..=3 => println!("{}: {} ns", data, time),
        4 => println!("{}: {:.3} μs", data, time / 1_000.0),
        5 => println!("{}: {:.2} μs", data, time / 1_000.0),
        6 => println!("{}: {:.1} μs", data, time / 1_000.0),
        7 => println!("{}: {:.3} ms", data, time / 1_000_000.0),
        8 => println!("{}: {:.2} ms", data, time / 1_000_000.0),
        9 => println!("{}: {:.1} ms", data, time / 1_000_000.0),
        10 => println!("{}: {:.3} s", data, time / 1_000_000_000.0),
        11 => println!("{}: {:.2} s", data, time / 1_000_000_000.0),
        12 => println!("{}: {:.1} s", data, time / 1_000_000_000.0),
        _ => println!("{}: {:.0} s", data, time / 1_000_000_000.0),
    }
}
