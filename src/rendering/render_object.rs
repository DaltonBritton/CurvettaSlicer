use three_d::{ColorMaterial, Gm, InstancedMesh, Instances, Mesh, Object};

pub enum RenderObject {
    TetNodes(Gm<InstancedMesh, ColorMaterial>, Instances),
    TetEdges(Gm<InstancedMesh, ColorMaterial>, Instances),
    TetBoundarySurface(Gm<Mesh, ColorMaterial>),
    TetSurface(Gm<Mesh, ColorMaterial>),
}

impl RenderObject {
    pub fn as_object(&self) -> &dyn Object {
        match self {
            RenderObject::TetNodes(gm, _) => gm,
            RenderObject::TetEdges(gm, _) => gm,
            RenderObject::TetBoundarySurface(gm) => gm,
            RenderObject::TetSurface(gm) => gm,
        }
    }
}
