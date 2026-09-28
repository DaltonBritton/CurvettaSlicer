#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TetVertexIndex {
    pub(in crate::data::tet_graph) index: usize,
}
impl TetVertexIndex {
    pub(in crate::data::tet_graph) fn _new(index: usize) -> Self {
        Self { index }
    }

    pub unsafe fn from_raw(index: usize) -> Self {
        Self { index }
    }
}
