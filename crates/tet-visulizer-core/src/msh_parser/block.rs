use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::value,
    error::ParseError,
    sequence::{preceded, terminated},
};

pub fn block<'a, O, E: ParseError<&'a [u8]>>(
    name: &'static str,
    inner: impl Parser<&'a [u8], Output = O, Error = E>,
) -> impl Parser<&'a [u8], Output = O, Error = E> {
    terminated(preceded(block_start(name), inner), block_end(name))
}

pub fn block_start<'a, E: ParseError<&'a [u8]>>(
    name: &'static str,
) -> impl Parser<&'a [u8], Output = (), Error = E> {
    value((), (char('$'), tag(name), line_ending))
}

pub fn block_end<'a, E: ParseError<&'a [u8]>>(
    name: &'static str,
) -> impl Parser<&'a [u8], Output = (), Error = E> {
    value((), (tag("$End"), tag(name), line_ending))
}
