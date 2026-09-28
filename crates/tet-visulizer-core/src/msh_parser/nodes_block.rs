use std::{collections::HashMap, hash::Hash};

use cgmath::Vector3;
use nom::{
    Parser,
    character::{
        self,
        complete::{line_ending, newline},
        streaming::space1,
    },
    combinator::map,
    error::ParseError,
    multi::{fold_many_m_n, length_count},
    number::{self, complete::double},
    sequence::{preceded, terminated},
};

use crate::msh_parser::block::block;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(super) u64);

#[derive(Debug, PartialEq)]
pub struct Node {
    id: NodeId,
    x: f64,
    y: f64,
    z: f64,
}

impl Node {
    fn new(id: NodeId, x: f64, y: f64, z: f64) -> Self {
        Self { id, x, y, z }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }
}

impl From<&Node> for Vector3<f64> {
    fn from(node: &Node) -> Self {
        Vector3 {
            x: node.x,
            y: node.y,
            z: node.z,
        }
    }
}

fn num_nodes<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = u64, Error = E> {
    terminated(character::complete::u64, newline)
}

fn node_id<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = NodeId, Error = E> {
    map(character::complete::u64, |id| NodeId(id))
}

fn coordinate<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = f64, Error = E> {
    number::complete::double
}

fn node<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = Node, Error = E> {
    map(
        (
            node_id(),
            space1,
            coordinate(),
            space1,
            coordinate(),
            space1,
            coordinate(),
            line_ending,
        ),
        |(node_id, _, x, _, y, _, z, _)| Node::new(node_id, x, y, z),
    )
}

fn nodes<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = Vec<Node>, Error = E> {
    length_count(num_nodes(), node())
}

pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = Vec<Node>, Error = E>
{
    block("Nodes", nodes())
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestErr<'a> = nom::error::Error<&'a [u8]>;

    #[test]
    fn parse_mesh_header_test() {
        let node_data = "$Nodes\n3\n1 -24.9958 -0.205066 5.48363e-06\n2 -24.919 1.99951 6.67572e-06\n3 -24.8893 -2.22594 5.11852e-06\n$EndNodes\n".as_bytes();

        let (_, nodes) = parser::<TestErr>().parse_complete(node_data).unwrap();

        assert_eq!(nodes.len(), 3);
    }
}
