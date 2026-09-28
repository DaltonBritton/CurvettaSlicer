use nom::{
    Parser,
    character::complete::{self, line_ending, newline, space1},
    combinator::{map, map_opt, verify},
    error::ParseError,
    multi::{count, length_count},
    sequence::terminated,
};

use crate::msh_parser::{block::block, nodes_block::NodeId};

enum MshElementType {
    Tet = 4,
}

pub struct MshTet {
    pub node_ids: [NodeId; 4],
}

fn num_elements<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = u64, Error = E> {
    terminated(complete::u64, newline)
}

fn element_id<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = u64, Error = E> {
    complete::u64
}

fn element_type<'a, E: ParseError<&'a [u8]>>()
-> impl Parser<&'a [u8], Output = MshElementType, Error = E> {
    map_opt(complete::u64, |element_type| match element_type {
        4 => Some(MshElementType::Tet),
        _ => None,
    })
}

fn node_id<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = NodeId, Error = E> {
    map(complete::u64, |node_id| NodeId(node_id))
}

fn num_tags<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = u64, Error = E> {
    complete::u64
}

fn element_line<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = MshTet, Error = E>
{
    map(
        (
            element_id(),
            space1,
            element_type(),
            space1,
            verify(num_tags(), |num_tags| *num_tags == 0),
            space1,
            count(terminated(node_id(), space1), 4),
            line_ending,
        ),
        |(_element_id, _, _element_type, _, _num_tags, _, nodes, _)| MshTet {
            node_ids: [nodes[0], nodes[1], nodes[2], nodes[3]],
        },
    )
}

pub fn parser<'a, E: ParseError<&'a [u8]>>()
-> impl Parser<&'a [u8], Output = Vec<MshTet>, Error = E> {
    block("Elements", length_count(num_elements(), element_line()))
}
