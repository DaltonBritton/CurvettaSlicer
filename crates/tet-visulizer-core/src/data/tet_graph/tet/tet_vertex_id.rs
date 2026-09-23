use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum TetVertexId {
    A,
    B,
    C,
    D,
}
