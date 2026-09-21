#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderMode {
    FullTets,
    TetGraphNodes,
    TetGraphEdges,
    TetGraphBoundaryFaces,
}
