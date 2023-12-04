//! Utility module defining methods for parsing plain text.

use std::{marker::PhantomData, str::FromStr};

use crate::{combinator::Repeated, Input, Parser};

/// See [`just`].
#[derive(Clone, Copy)]
struct Just {
    token: &'static str,
}

impl<'a> Parser<'a, &'a str, ()> for Just {
    fn parse(&self, input: &mut Input<'a, &'a str>) -> Result<(), ()> {
        if input.source[input.pos..input.pos + self.token.len()] == *self.token {
            input.pos += self.token.len();
            Ok(())
        } else {
            Err(())
        }
    }
}

/// A parser that matches a single instance of the given token.
///
/// This can be used in combination with [`Parser::repeated`] to parse a sequence
/// of one or more instances of the given token.
///
/// # Example
///
/// ```
/// let parser = just("hello").then(repeated(just("world")));
/// let input = "helloworldworld";
/// parser.parse(input.into());
/// ```
pub fn just<'a>(token: &'static str) -> impl Parser<'a, &'a str, ()> {
    Just { token }
}

#[derive(Clone, Copy)]
pub struct OneOf {
    tokens: &'static [&'static str],
}

impl<'a> Parser<'a, &'a str, &'a str> for OneOf {
    fn parse(&self, input: &mut Input<'a, &'a str>) -> Result<&'a str, ()> {
        for token in self.tokens {
            if input.source[input.pos..input.pos + token.len()] == **token {
                input.pos += token.len();
                return Ok(token);
            }
        }
        Err(())
    }
}

/// Returns a parser that matches any of the given tokens.
pub fn one_of<'a>(tokens: &'static [&'static str]) -> impl Parser<'a, &'a str, &'a str> {
    OneOf { tokens }
}

/// See [`ascii`].
#[derive(Clone, Copy)]
pub struct Ascii;

impl<'a> Parser<'a, &'a str, &'a str> for Ascii {
    fn parse(&self, input: &mut Input<'a, &'a str>) -> Result<&'a str, ()> {
        let mut pos = input.pos;
        while pos < input.source.len() {
            if input.source[pos..pos + 1]
                .chars()
                .next()
                .unwrap()
                .is_ascii()
            {
                pos += 1;
            } else {
                break;
            }
        }
        if pos == input.pos {
            Err(())
        } else {
            input.pos = pos;
            Ok(&input.source[input.pos..pos])
        }
    }
}

/// A parser that matches one or more ASCII characters.
pub fn ascii<'a>() -> impl Parser<'a, &'a str, &'a str> {
    Ascii
}

/// See [`number`].
#[derive(Clone, Copy)]
pub struct Number<T> {
    pub radix: T,
    pub __phantom: PhantomData<T>,
}

macro_rules! unsigned_number_impl {
    ($t:ty) => {
        impl<'a> Parser<'a, &'a str, $t> for Number<$t> {
            fn parse(&self, input: &mut Input<'a, &'a str>) -> Result<$t, ()> {
                let mut pos = input.pos;
                let mut value = 0;
                while pos < input.source.len() {
                    let digit = input.source[pos..pos + 1].parse::<$t>().map_err(|_| ())?;
                    if digit >= self.radix {
                        break;
                    }
                    value = value * self.radix + digit;
                    pos += 1;
                }
                if pos == input.pos {
                    Err(())
                } else {
                    input.pos = pos;
                    Ok(value)
                }
            }
        }
    };
}

unsigned_number_impl!(u8);
unsigned_number_impl!(u16);
unsigned_number_impl!(u32);
unsigned_number_impl!(u64);
unsigned_number_impl!(u128);
unsigned_number_impl!(usize);

macro_rules! signed_number_impl {
    ($t:ty) => {
        impl<'a> Parser<'a, &'a str, $t> for Number<$t> {
            fn parse(&self, input: &mut Input<'a, &'a str>) -> Result<$t, ()> {
                let mut pos = input.pos;
                let mut value = 0;
                let mut sign = 1;
                if input.source[pos..pos + 1] == *"-" {
                    sign = -1;
                    pos += 1;
                }
                while pos < input.source.len() {
                    let digit = input.source[pos..pos + 1].parse::<$t>().map_err(|_| ())?;
                    if digit >= self.radix {
                        break;
                    }
                    value = value * self.radix + digit;
                    pos += 1;
                }
                if pos == input.pos {
                    Err(())
                } else {
                    input.pos = pos;
                    Ok(value * sign)
                }
            }
        }
    };
}

signed_number_impl!(i8);
signed_number_impl!(i16);
signed_number_impl!(i32);
signed_number_impl!(i64);
signed_number_impl!(i128);

/// A parser that matches a number in the given radix.
///
/// This parser is generic, and is capable of parsing numbers into all signed and
/// unsigned integer primitives.
///
/// # Example
/// ```
/// let integer = number::<u32>();
/// ```
pub fn number<T>(radix: T) -> Number<T>
where
    T: FromStr,
{
    Number {
        radix,
        __phantom: PhantomData,
    }
}

/// A utility parser that consumes whitespace.
pub fn whitespace<'a>() -> Repeated<impl Parser<'a, &'a str, ()>, ()> {
    one_of(&["\t", "\r", "\n", " "]).ignored().repeated()
}
