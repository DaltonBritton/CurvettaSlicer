use three_d::{ColorMaterial, Gm, InstancedMesh, Mesh, Object};

pub enum RenderObject {
    TetNodes(Gm<InstancedMesh, ColorMaterial>),
    TetEdges(Gm<InstancedMesh, ColorMaterial>),
    TetBoundarySurface(Gm<Mesh, ColorMaterial>),
    TetSurface(Gm<Mesh, ColorMaterial>),
}

impl RenderObject {
    pub fn as_object(&self) -> &dyn Object {
        match self {
            RenderObject::TetNodes(gm) => gm,
            RenderObject::TetEdges(gm) => gm,
            RenderObject::TetBoundarySurface(gm) => gm,
            RenderObject::TetSurface(gm) => gm,
        }
    }
}
