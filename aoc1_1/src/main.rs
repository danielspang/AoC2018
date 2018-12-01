use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    println!(
        "{}",
        stdin
            .lock()
            .lines()
            .map(|s| s.unwrap().parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
