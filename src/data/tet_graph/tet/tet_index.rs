#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TetIndex {
    pub(in crate::data::tet_graph) index: usize,
}
impl TetIndex {
    pub(crate) fn new(index: usize) -> Self {
        Self { index }
    }
}
