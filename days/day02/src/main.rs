type Score = (u8, u8, u8);

enum Color {
    Red,
    Green,
    Blue,
}

struct ScoreParser<'a> {
    content: &'a str,
    pos: usize,
}

impl<'a> ScoreParser<'a> {
    fn score(&mut self) -> Option<u8> {
        let mut out = 0;
        while let Some(c) = self.content.chars().nth(self.pos) {
            if let Some(d) = c.to_digit(10) {
                out = out * 10 + d as u8;
                self.pos += 1;
            } else {
                break;
            }
        }
        if out == 0 {
            None
        } else {
            Some(out)
        }
    }
    fn color(&mut self) -> Option<Color> {
        let mut out = String::new();
        while let Some(c) = self.content.chars().nth(self.pos) {
            if c.is_alphabetic() {
                out.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        match out.as_str() {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }

    fn whitespace(&mut self) {
        while let Some(c) = self.content.chars().nth(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn eat(&mut self, s: &str) -> Option<()> {
        if self.content[self.pos..self.pos + s.len()] == *s {
            self.pos += s.len();
            Some(())
        } else {
            None
        }
    }

    fn peek_boundary(&self) -> bool {
        return self.content.chars().nth(self.pos).unwrap() == ';'
            || self.pos == self.content.len();
    }

    fn score_color(&mut self) -> Option<(Score, Color)> {
        let score = self.score().and(|r| {
            self.whitespace();
            self.color()
        });
    }

    fn many<T>(
        &mut self,
        parser: impl Fn(&mut Self) -> Option<T>,
        delimiter: &'static str,
    ) -> Vec<T> {
        let out = vec![];
        for _ in 0.. {
            if let Some(t) = parser(self) {
                out.push(t);
                if self.eat(delimiter) {
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        out
    }

    fn game(&mut self) -> Option<Score> {
        self.eat("Game:")
            .and_then(|_| self.whitespace())
            .and_then(|_| self.many(self.score_color, ";"))
    }
}

impl<'a> Iterator for ScoreParser<'a> {
    type Item = Score;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {}
