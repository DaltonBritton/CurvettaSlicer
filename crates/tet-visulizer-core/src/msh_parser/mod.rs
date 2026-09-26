use nom::{Parser, error::ParseError};

use crate::data::TetGraph;

mod block;
mod mesh_format_block;
mod nodes_block;

pub fn parse_msh() {}

pub fn msh<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = TetGraph, Error = E> {
    (mesh_format_block::parser(), nodes_block::parser());

    todo!();
}
