use crate::data::tet_graph::tet::TetVertexIndex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TetFace {
    pub(in crate::data::tet_graph) points: [TetVertexIndex; 3],
}

impl TetFace {
    pub fn new(a: TetVertexIndex, b: TetVertexIndex, c: TetVertexIndex) -> TetFace {
        let mut points = [a, b, c];
        points.sort();

        Self { points }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, TetVertexIndex> {
        self.points.iter()
    }
}
