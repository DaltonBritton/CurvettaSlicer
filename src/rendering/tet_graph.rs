use three_d::{
    CpuMesh, InnerSpace, Instances, Mat4, MetricSpace, Quaternion, Srgba, Vec3, Vector3,
};

use crate::{
    data::TetGraph,
    rendering::utils::{random_color, remap_point_yz_axis},
};

fn vec_cast(Vector3 { x, y, z }: Vector3<f64>) -> Vector3<f32> {
    Vector3 {
        x: x as f32,
        y: y as f32,
        z: z as f32,
    }
}

impl TetGraph {
    pub fn _render_tets(&self) {}
    pub fn _render_tets_as_nodes(&self) -> Instances {
        let nodes = self.get_nodes();

        let node_instance_transforms = nodes
            .iter()
            .map(|node| node.center())
            .map(|center| remap_point_yz_axis(center))
            .map(|center| vec_cast(center))
            .map(|center| Mat4::from_translation(center) * Mat4::from_scale(0.4))
            .collect();

        Instances {
            transformations: node_instance_transforms,
            texture_transformations: None,
            colors: None,
        }
    }
    pub fn _render_boundary_faces(&self) -> CpuMesh {
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
                mesh_points.push(remap_point_yz_axis(self.get_point(point_index)));

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
    pub fn _render_neighbor_edges(&self) -> Instances {
        let edges = self.get_neighbor_edges();
        let mut edge_transforms = Vec::with_capacity(edges.len());

        for edge in edges {
            let neighbor_a_center = remap_point_yz_axis(self.get_node(edge.neighbor_a).center());
            let neighbor_b_center = remap_point_yz_axis(self.get_node(edge.neighbor_b).center());

            let dir = (neighbor_b_center - neighbor_a_center).normalize();

            let dist = neighbor_a_center.distance(neighbor_b_center);

            let scale = Mat4::from_nonuniform_scale(dist as f32, 0.1, 0.1);
            let rotation = Mat4::from(Quaternion::from_arc(Vector3::unit_x(), vec_cast(dir), None));
            let translation = Mat4::from_translation(vec_cast(neighbor_a_center));

            let transform = translation * rotation * scale;

            edge_transforms.push(transform);
        }

        Instances {
            transformations: edge_transforms,
            texture_transformations: None,
            colors: None,
        }
    }
}
