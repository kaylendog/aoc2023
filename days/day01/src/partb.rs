use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn process_word<S: AsRef<str>>(content: S) -> u64 {
    let content = content.as_ref();
    // single digit
    if content.len() == 1 {
        return content.chars().next().unwrap().to_digit(10).unwrap() as u64;
    };
    // words
    match content {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("invalid input"),
    }
}

struct LineIterator {
    buf: String,
    reader: BufReader<File>,
}

impl From<File> for LineIterator {
    fn from(file: File) -> Self {
        Self {
            buf: String::with_capacity(128),
            reader: BufReader::new(file),
        }
    }
}

impl Iterator for LineIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_line(&mut self.buf) {
            Ok(n) => {
                if n == 0 {
                    return None;
                }
                let out = Some(self.buf.clone());
                self.buf.drain(0..);
                out
            }
            Err(_) => panic!("failed to read line"),
        }
    }
}

fn part2<P: AsRef<Path>>(path: P) -> u64 {
    let file = std::fs::File::open(path.as_ref()).expect("failed to open input");
    let lines: LineIterator = file.into();
    let regex = Regex::new(r"^(.*?(\d|one|two|three|four|five|six|seven|eight|nine))?.*(\d|one|two|three|four|five|six|seven|eight|nine).*$").expect("failed to compile regex");

    lines
        .map(|line| {
            let captures = regex
                .captures(&line[0..line.len() - 1])
                .expect("invalid input");
            let lsd = captures
                .get(3)
                .map(|word| process_word(word.as_str()))
                .expect("invalid input");
            let msd = captures
                .get(2)
                .map_or(lsd, |word| process_word(word.as_str()))
                * 10;
            msd + lsd
        })
        .reduce(|acc, v| acc + v)
        .unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("no path passed");
    println!("b: {}", part2(path));
}
