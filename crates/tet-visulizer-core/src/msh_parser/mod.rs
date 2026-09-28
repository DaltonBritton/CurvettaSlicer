use nom::{Parser, combinator::map, error::ParseError};

use crate::data::TetGraph;

mod block;
mod elements_block;
mod mesh_format_block;
mod nodes_block;

pub fn parse_msh() {}

pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = TetGraph, Error = E>
{
    map(
        (
            mesh_format_block::parser(),
            nodes_block::parser(),
            elements_block::parser(),
        ),
        |(_, nodes, tets)| {
            TetGraph::new(nodes.into_values().map(|node| node.into()).collect(), &tets)
        },
    )
}
