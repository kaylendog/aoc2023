use aocparse::{
    text::{just, number},
    Parser,
};

enum Op {
    Integer(i32),
    Plus(Box<Op>, Box<Op>),
    Times(Box<Op>, Box<Op>),
}

#[test]
fn parse_expr() {
    // create parser
    let integer = number(10).map(Op::Integer);
    let mul = integer
        .then(just("*").ignored())
        .then(integer)
        .map(|((a, _), b)| Op::Times(Box::new(a), Box::new(b)));

    let atom = integer.or(mul.clone());

    let add = atom
        .clone()
        .then(just("+").ignored())
        .then(atom.clone())
        .map(|((a, _), b)| Op::Plus(Box::new(a), Box::new(b)));

    let parser = add.or(mul).or(atom);
}
