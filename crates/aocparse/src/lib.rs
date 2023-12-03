//! Generic parsing utilities for Advent of Code.

use std::marker::PhantomData;

use combinator::{DelimitedBy, Maybe, Or, Repeated, Then};
use primitive::{Ignored, Map, To};

mod primitive;

pub mod combinator;
pub mod text;

pub struct Input<'a, I> {
    pos: usize,
    source: I,
    __phantom: PhantomData<&'a I>,
}

impl<'a> From<&'a str> for Input<'a, &'a str> {
    fn from(source: &'a str) -> Self {
        Input {
            pos: 0,
            source,
            __phantom: PhantomData,
        }
    }
}

/// A trait for parsing input into a desired output.
pub trait Parser<'a, I, O>: Clone {
    /// Ignore the result of the parser.
    fn ignored(self) -> Ignored<Self, O>
    where
        Self: Sized,
    {
        Ignored {
            parser: self,
            value: (),
            __phantom: PhantomData,
        }
    }

    /// Maps the result of this parser to the desired output.
    fn map<T, F>(self, f: F) -> Map<Self, F, O>
    where
        Self: Sized,
        F: Fn(O) -> T,
    {
        Map {
            f,
            parser: self,
            __phantom: PhantomData,
        }
    }

    /// Use this parser, or another if this one fails.
    fn or<B>(self, other: B) -> Or<Self, B>
    where
        Self: Sized,
        B: Parser<'a, I, O>,
    {
        Or { a: self, b: other }
    }

    /// Map the output of this parser to the given constant value.
    fn to<T>(self, value: T) -> To<Self, O, T>
    where
        Self: Sized,
    {
        To {
            parser: self,
            value,
            __phantom: PhantomData,
        }
    }

    /// Use this parser, then another.
    fn then<B>(self, other: B) -> Then<Self, B>
    where
        Self: Sized,
    {
        Then { a: self, b: other }
    }

    /// Make this parser optional.
    fn optional(self) -> Maybe<Self>
    where
        Self: Sized,
    {
        Maybe { parser: self }
    }

    /// Repeat this parser until it fails.
    fn repeated(self) -> Repeated<Self, O>
    where
        Self: Sized,
    {
        Repeated {
            parser: self,
            __phantom: PhantomData,
        }
    }

    fn delimited_by<D>(self, delimiter: D) -> DelimitedBy<Self, D>
    where
        Self: Sized,
    {
        DelimitedBy {
            item: self,
            delimiter,
        }
    }

    /// Parse the given input.
    fn parse(&self, input: &mut Input<'a, I>) -> Result<O, ()>;
}

/// Provides the ability to treat boxed parsers as if they were not boxed.
impl<'a, I, O, P> Parser<'a, I, O> for Box<P>
where
    P: Parser<'a, I, O>,
    I: Clone,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<O, ()> {
        self.as_ref().parse(input)
    }
}
