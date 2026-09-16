use three_d::{CpuMesh, Srgba, Vector3};

use crate::{
    data::TetGraph,
    rendering::utils::{random_color, remap_point_yz_axis},
};

impl TetGraph {
    pub fn _render_tets(&self) {}
    pub fn _render_tets_as_nodes(&self) {}
    pub fn _render_boundary_faces(&self) -> CpuMesh {
        let tet_points = self.get_points();
        let boundary_faces = self.get_boundary_faces();

        let mut mesh_points: Vec<Vector3<f64>> = Vec::with_capacity(boundary_faces.len() * 3);
        let mut triangles: Vec<u32> = Vec::with_capacity(boundary_faces.len());
        let mut colors: Vec<Srgba> = Vec::with_capacity(boundary_faces.len() * 3);

        //used for the color generation
        let mut rng = rand::rng();

        for (i, (face, _tet)) in boundary_faces.iter().enumerate() {
            let color = random_color(&mut rng);

            for (j, point_index) in face.iter().enumerate() {
                let point_index = *point_index;
                mesh_points.push(remap_point_yz_axis(tet_points[point_index]));

                triangles.push((i * 3 + j) as u32);

                colors.push(color);
            }
        }

        let mesh = CpuMesh {
            positions: three_d::Positions::F64(mesh_points),
            indices: three_d::Indices::U32(triangles),
            colors: Some(colors),
            ..Default::default()
        };

        mesh
    }
    pub fn _render_neighbor_edges(&self) {}
}
