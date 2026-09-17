use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::data::tet_graph::tet::{TetFace, TetVertexId, TetVertexIndex};

#[derive(Debug, Clone, Copy)]
pub struct Tet {
    pub(super) points: [TetVertexIndex; 4],
}

impl Tet {
    pub fn new(points: [TetVertexIndex; 4]) -> Tet {
        Tet { points }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, TetVertexIndex> {
        self.points.iter()
    }

    pub fn get_point(&self, vertex: TetVertexId) -> TetVertexIndex {
        match vertex {
            TetVertexId::A => self.points[0],
            TetVertexId::B => self.points[1],
            TetVertexId::C => self.points[2],
            TetVertexId::D => self.points[3],
        }
    }

    pub fn get_face_across_from_vertex(&self, vertex: TetVertexId) -> TetFace {
        match vertex {
            TetVertexId::A => TetFace::new(
                self.get_point(TetVertexId::B),
                self.get_point(TetVertexId::C),
                self.get_point(TetVertexId::D),
            ),
            TetVertexId::B => TetFace::new(
                self.get_point(TetVertexId::A),
                self.get_point(TetVertexId::C),
                self.get_point(TetVertexId::D),
            ),
            TetVertexId::C => TetFace::new(
                self.get_point(TetVertexId::A),
                self.get_point(TetVertexId::B),
                self.get_point(TetVertexId::D),
            ),
            TetVertexId::D => TetFace::new(
                self.get_point(TetVertexId::A),
                self.get_point(TetVertexId::B),
                self.get_point(TetVertexId::C),
            ),
        }
    }

    pub fn get_vertex_across_from_face(&self, face: TetFace) -> Option<TetVertexId> {
        for vertex_id in TetVertexId::iter() {
            if !face.points.contains(&self.get_point(vertex_id)) {
                return Some(vertex_id);
            }
        }

        None
    }
}

impl Display for Tet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tet {:?}", self.points)
    }
}
