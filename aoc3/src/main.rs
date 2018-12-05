#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::{self, Read};

use regex::Regex;

#[derive(Debug, PartialEq)]
struct Claim {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

fn iter_from_str<'a>(input: &'a str) -> impl Iterator<Item = Claim> + 'a {
    // #6 @ 366,229: 28x28
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    }
    RE.captures_iter(input)
        .map(|cap| (1..6).map(|i| cap[i].parse::<usize>().unwrap()).collect())
        .map(|cap: Vec<usize>| Claim {
            id: cap[0],
            x: cap[1],
            y: cap[2],
            w: cap[3],
            h: cap[4],
        })
}

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut counts: Vec<Vec<u8>> = vec![vec![0; 1000]; 1000];
    iter_from_str(&input).for_each(|claim| {
        counts
            .iter_mut()
            .skip(claim.y)
            .take(claim.h)
            .flat_map(|row| row.iter_mut().skip(claim.x).take(claim.w))
            .for_each(|count| *count += 1);
    });

    let overlap = counts.iter().flatten().filter(|&&count| count > 1).count();
    println!("{}", overlap);

    iter_from_str(&input).for_each(|claim| {
        if counts
            .iter()
            .skip(claim.y)
            .take(claim.h)
            .flat_map(|row| row.iter().skip(claim.x).take(claim.w))
            .all(|count| *count == 1)
        {
            println!("{}", claim.id);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_line() {
        assert_eq!(
            iter_from_str("#6 @ 366,229: 28x29\n#6 @ 366,229: 28x29").collect::<Vec<Claim>>(),
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
