use std::path::Path;

use regex::Regex;
use utils::{process_word, LineIterator};

mod utils;

fn part1<P: AsRef<Path>>(path: P) -> u64 {
    let file = std::fs::File::open(path.as_ref()).expect("failed to open input");
    let lines: LineIterator = file.into();
    let regex = Regex::new(r"^[^\d]*(\d)?.*(\d)[^\d]*$").expect("failed to compile regex");

    lines
        .map(|line| {
            let captures = regex
                .captures(&line[0..line.len() - 1])
                .expect("invalid input");
            let lsd = captures
                .get(2)
                .map(|word| process_word(word.as_str()))
                .expect("invalid input");
            let msd = captures
                .get(1)
                .map_or(lsd, |word| process_word(word.as_str()))
                * 10;
            msd + lsd
        })
        .reduce(|acc, v| acc + v)
        .unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("no path passed");
    println!("a: {}", part1(path));
}
