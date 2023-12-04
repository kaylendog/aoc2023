use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

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

/// Forward and backward pass method - does a fair amount
/// of backtracking - will speed up with RadixTree eventually.
fn part2<P: AsRef<Path>>(path: P) -> u64 {
    let lines = BufReader::new(File::open(path.as_ref()).unwrap()).lines();
    let words = HashSet::from([
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]);

    lines
        .map(Result::unwrap)
        .map(|line| {
            let mut msd = 0;
            let mut lsd = None;

            // front pass
            'front: for (pos, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    msd = c.to_digit(10).unwrap() as u64;
                    break;
                }
                // search for a word
                let mut words = words.clone();
                for i in pos..line.len() {
                    words.retain(|el| el.starts_with(&line[pos..i]));
                    if words.is_empty() {
                        continue 'front;
                    }
                    if let Some(word) = words.get(&line[pos..i]) {
                        msd = process_word(word);
                        break 'front;
                    }
                }
            }
            // back pass
            'back: for (pos, c) in line.chars().rev().enumerate() {
                let pos = line.len() - pos;
                if c.is_ascii_digit() {
                    lsd = Some(c.to_digit(10).unwrap() as u64);
                    break;
                }
                // search for a word
                let mut words = words.clone();
                for i in (0..pos).rev() {
                    words.retain(|el| el.ends_with(&line[i..pos]));
                    if words.is_empty() {
                        continue 'back;
                    }
                    if let Some(word) = words.get(&line[i..pos]) {
                        lsd = Some(process_word(word));
                        break 'back;
                    }
                }
            }
            msd * 10 + lsd.unwrap_or(msd)
        })
        .reduce(|acc, v| acc + v)
        .unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("no path passed");
    println!("{}", part2(path));
}
