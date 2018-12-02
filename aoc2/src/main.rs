use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

fn two_three(s: &String) -> (i32, i32) {
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

    let mut iter = [2, 3].iter().map(|freq| {
        map.values()
            .filter_map(|x| if x == freq { Some(1) } else { None })
            .next()
            .unwrap_or(0)
    });

    (iter.next().unwrap(), iter.next().unwrap())
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|s| s.ok()).collect();

    let a: (i32, i32) = lines
        .iter()
        .map(two_three)
        .fold((0, 0), |(a2, a3), (x2, x3)| (a2 + x2, a3 + x3));
    println!("{}", a.0 * a.1);

    let id_len = lines[0].len();
    for i in 0..id_len {
        if let Some(Some(res)) = lines
            .iter()
            .cloned()
            .map(|mut s| {
                s.remove(i);
                s
            }).scan(HashSet::new(), |set, s| {
                if set.contains(&s) {
                    Some(Some(s))
                } else {
                    set.insert(s);
                    Some(None)
                }
            }).filter(|s| s.is_some())
            .next()
        {
            println!("{}", res);
        };
    }
}
