use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn process_word<S: AsRef<str>>(content: S) -> u64 {
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

pub(crate) struct LineIterator {
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
