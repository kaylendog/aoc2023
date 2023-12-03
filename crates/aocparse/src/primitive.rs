//! Defines core primitive parsers.

use std::marker::PhantomData;

use crate::{Input, Parser};

/// A parser that allows for the mapping of its child output to another type.
#[derive(Copy)]
pub struct Map<P, F, O> {
    pub parser: P,
    pub f: F,
    pub __phantom: PhantomData<O>,
}

impl<P: Clone, F: Clone + Copy, O> Clone for Map<P, F, O> {
    fn clone(&self) -> Self {
        Map {
            parser: self.parser.clone(),
            f: self.f,
            __phantom: PhantomData,
        }
    }
}

impl<'a, P, AI, AO, BO, F: Clone + Copy> Parser<'a, AI, BO> for Map<P, F, AO>
where
    P: Parser<'a, AI, AO> + Sized,
    F: Fn(AO) -> BO,
{
    fn parse(&self, input: &mut Input<'a, AI>) -> Result<BO, ()> {
        self.parser.parse(input).map(|o| (self.f)(o))
    }
}

/// A parser that maps its child output to `()`.
pub type Ignored<P, O> = To<P, O, ()>;

/// A parser that maps its child output to a constant value.
pub struct To<P, O, T> {
    pub parser: P,
    pub value: T,
    pub __phantom: PhantomData<O>,
}

impl<P, O, T> Clone for To<P, O, T>
where
    P: Clone,
    T: Clone,
{
    fn clone(&self) -> Self {
        To {
            parser: self.parser.clone(),
            value: self.value.clone(),
            __phantom: PhantomData,
        }
    }
}

impl<'a, P, I, O, T> Parser<'a, I, T> for To<P, O, T>
where
    P: Parser<'a, I, O> + Sized,
    T: Clone,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<T, ()> {
        self.parser.parse(input).map(|_| self.value.clone())
    }
}

/// A parser that allows for the filtering of its child output.
struct Filter<P, F> {
    parser: P,
    f: F,
}

impl<P: Clone, F: Clone + Copy> Clone for Filter<P, F> {
    fn clone(&self) -> Self {
        Filter {
            parser: self.parser.clone(),
            f: self.f,
        }
    }
}

impl<'a, P, I, O, F: Clone + Copy> Parser<'a, I, O> for Filter<P, F>
where
    P: Parser<'a, I, O> + Sized,
    F: Fn(&O) -> bool,
{
    fn parse(&self, input: &mut Input<'a, I>) -> Result<O, ()> {
        let result = self.parser.parse(input)?;
        if (self.f)(&result) {
            Ok(result)
        } else {
            Err(())
        }
    }
}
