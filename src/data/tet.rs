use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tet {
    pub(super) points: [usize; 4],
}

impl Display for Tet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tet {:?}", self.points)
    }
}

impl Tet {
    pub fn new(points: [usize; 4]) -> Tet {
        Tet { points }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.points.iter()
    }
}
