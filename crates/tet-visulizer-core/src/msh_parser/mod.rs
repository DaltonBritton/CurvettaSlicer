use std::{collections::HashMap, error::Error, fmt::Display};

use cgmath::Vector3;
use nom::{Parser, combinator::map, error::ParseError};

use crate::{
    data::{Tet, TetGraph, TetVertexIndex},
    msh_parser::{
        elements_block::MshTet,
        nodes_block::{Node, NodeId},
    },
};

mod block;
mod elements_block;
mod mesh_format_block;
mod nodes_block;

pub struct MshMesh {
    nodes: Vec<Node>,
    tets: Vec<MshTet>,
}

#[derive(Debug)]
pub enum MshError {
    UnknownNode(NodeId),
}

impl Display for MshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            MshError::UnknownNode(node_id) => format!("Unknown Node: {:?}", node_id),
        };

        f.write_str(&error_message)
    }
}

impl Error for MshError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl TryFrom<MshMesh> for TetGraph {
    type Error = MshError;

    fn try_from(msh: MshMesh) -> Result<Self, Self::Error> {
        let mut index_of: HashMap<NodeId, usize> = HashMap::with_capacity(msh.nodes.len());
        let mut points: Vec<Vector3<f64>> = Vec::with_capacity(msh.nodes.len());
        let mut tets: Vec<Tet> = Vec::with_capacity(msh.tets.len());

        for (i, node) in msh.nodes.iter().enumerate() {
            points.push(node.into());
            index_of.insert(node.id(), i);
        }

        let lookup = |id: NodeId| {
            index_of
                .get(&id)
                .copied()
                .map(|index| unsafe { TetVertexIndex::from_raw(index) })
                .ok_or(MshError::UnknownNode(id))
        };

        for msh_tet in &msh.tets {
            let [a, b, c, d] = msh_tet.node_ids;
            tets.push(Tet::new([lookup(a)?, lookup(b)?, lookup(c)?, lookup(d)?]));
        }

        Ok(TetGraph::new(points, &tets))
    }
}

pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Parser<&'a [u8], Output = MshMesh, Error = E> {
    map(
        (
            mesh_format_block::parser(),
            nodes_block::parser(),
            elements_block::parser(),
        ),
        |(_, nodes, tets)| MshMesh { nodes, tets },
    )
}
