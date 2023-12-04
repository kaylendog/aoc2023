//! Defines combinators for composing parsers.

use std::marker::PhantomData;

use crate::{Input, Parser};

/// A parser that allows for sequencing of two child parsers.
#[derive(Clone)]
pub struct Then<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<'a, A, B, I, OA, OB> Parser<'a, I, (OA, OB)> for Then<A, B>
where
    A: Parser<'a, I, OA>,
    B: Parser<'a, I, OB>,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<(OA, OB), ()> {
        let a = self.a.parse(input)?;
        let b = self.b.parse(input)?;
        Ok((a, b))
    }
}

/// A parser that allows for optional parsing using its child parser.
#[derive(Clone)]
pub struct Maybe<P> {
    pub(crate) parser: P,
}

impl<'a, I, O, P> Parser<'a, I, Option<O>> for Maybe<P>
where
    P: Parser<'a, I, O>,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<Option<O>, ()> {
        match self.parser.parse(input) {
            Ok(o) => Ok(Some(o)),
            Err(_) => Ok(None),
        }
    }
}

/// A parser that allows for repeated parsing using its child parser.
pub struct Repeated<P, O> {
    pub(crate) parser: P,
    pub(crate) __phantom: PhantomData<O>,
}

impl<P: Clone, O> Clone for Repeated<P, O> {
    fn clone(&self) -> Self {
        Repeated {
            parser: self.parser.clone(),
            __phantom: PhantomData,
        }
    }
}

impl<'a, I, O, P> Parser<'a, I, Vec<O>> for Repeated<P, O>
where
    P: Parser<'a, I, O>,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<Vec<O>, ()> {
        let mut result = Vec::new();
        while let Ok(o) = self.parser.parse(input) {
            result.push(o);
        }
        Ok(result)
    }
}

/// A parser that allows for repeated parsing using its child parser a fixed number of times.
pub struct Repeat<P, O> {
    pub(crate) count: usize,
    pub(crate) parser: P,
    pub(crate) __phantom: PhantomData<O>,
}

impl<P, O> Clone for Repeat<P, O>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        Repeat {
            count: self.count,
            parser: self.parser.clone(),
            __phantom: PhantomData,
        }
    }
}

impl<'a, I, O, P> Parser<'a, I, Vec<O>> for Repeat<P, O>
where
    P: Parser<'a, I, O>,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<Vec<O>, ()> {
        let mut result = Vec::with_capacity(self.count);
        for _ in 0..self.count {
            result.push(self.parser.parse(input)?);
        }
        Ok(result)
    }
}

/// A parser that first tries to parse with `a`, then `b`.
#[derive(Clone)]
pub struct Or<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
}

impl<'a, A, B, I, O> Parser<'a, I, O> for Or<A, B>
where
    A: Parser<'a, I, O> + Sized,
    B: Parser<'a, I, O> + Sized,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<O, ()> {
        self.a.parse(input).or_else(|_| self.b.parse(input))
    }
}

/// See [Parser::foldl].
#[derive(Copy)]
pub struct Foldl<P, F, O> {
    pub(crate) parser: P,
    pub(crate) f: F,
    pub(crate) __phantom: PhantomData<O>,
}

impl<P, F, O> Clone for Foldl<P, F, O>
where
    P: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Foldl {
            parser: self.parser.clone(),
            f: self.f.clone(),
            __phantom: PhantomData,
        }
    }
}

impl<'a, P, I, O, F> Parser<'a, I, O> for Foldl<P, F, O>
where
    P: Parser<'a, I, Vec<O>>,
    F: Fn(O, O) -> O + Clone + Copy,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<O, ()> {
        self.parser.parse(input).map(|mut v| {
            // TODO: fails if v is empty
            let mut result = v.pop().unwrap();
            for o in v {
                result = (self.f)(result, o);
            }
            result
        })
    }
}

/// See [Parser::delimited_by].
#[derive(Clone)]
pub struct DelimitedBy<A, D> {
    pub(crate) item: A,
    pub(crate) delimiter: D,
}

impl<'a, A, D, I, O> Parser<'a, I, Vec<O>> for DelimitedBy<A, D>
where
    A: Parser<'a, I, O>,
    D: Parser<'a, I, ()>,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<Vec<O>, ()> {
        let mut items = self.item.parse(input).map_or(vec![], |v| vec![v]);
        let mut prev_pos = input.pos;
        loop {
            self.delimiter.parse(input)?;
            match self.item.parse(input) {
                Ok(item) => {
                    items.push(item);
                    prev_pos = input.pos;
                }
                Err(_) => {
                    // rewind to last successful parse
                    input.pos = prev_pos;
                    break;
                }
            }
        }

        Ok(items)
    }
}
