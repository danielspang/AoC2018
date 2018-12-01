use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut iter = lines
        .iter()
        .cycle()
        .scan(0, |sum, x| {
            *sum += x;
            Some(*sum)
        }).scan(HashSet::new(), |set, x| match set.contains(&x) {
            true => Some(Some(x)),
            false => {
                set.insert(x);
                Some(None)
            }
        }).filter(Option::is_some);
    println!("{}", iter.next().unwrap().unwrap());
}
