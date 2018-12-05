extern crate regex;

use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

fn from_line(input: &str) -> Vec<Claim> {
    // #6 @ 366,229: 28x28
    let re = Regex::new(r"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    re.captures_iter(input)
        .map(|cap| (1..6).map(|i| cap[i].parse::<u32>().unwrap()).collect())
        .map(|cap: Vec<u32>| Claim {
            id: cap[0],
            x: cap[1],
            y: cap[2],
            w: cap[3],
            h: cap[4],
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|s| s.ok()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        assert_eq!(
            from_line("#6 @ 366,229: 28x29\n#6 @ 366,229: 28x29"),
            vec![
                Claim {
                    id: 6,
                    x: 366,
                    y: 229,
                    w: 28,
                    h: 29,
                },
                Claim {
                    id: 6,
                    x: 366,
                    y: 229,
                    w: 28,
                    h: 29,
                },
            ]
        );
    }
}
