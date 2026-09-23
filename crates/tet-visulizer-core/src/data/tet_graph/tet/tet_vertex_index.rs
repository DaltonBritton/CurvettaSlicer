#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TetVertexIndex {
    pub(in crate::data::tet_graph) index: usize,
}
impl TetVertexIndex {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }
}
