use std::fmt::{Display, Write};

use rand::rng;
use three_d::Instances;

use crate::{
    data::TetGraph,
    rendering::{render_object::RenderObject, utils},
};

pub enum ColorMode {
    Random,
    EdgeLength,
    DistToGround,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_mode_name = match self {
            ColorMode::Random => "Random",
            ColorMode::EdgeLength => "Edge Length",
            ColorMode::DistToGround => "Dist To Ground",
        };

        f.write_str(color_mode_name)
    }
}

impl ColorMode {
    pub fn color_objects(&self, tet_graph: &TetGraph, objects: &mut [RenderObject]) {
        match self {
            ColorMode::Random => color_random(tet_graph, objects),
            ColorMode::EdgeLength => color_edge_length(tet_graph, objects),
            ColorMode::DistToGround => color_dist_to_ground(tet_graph, objects),
        }
    }
}

fn color_random(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    for obj in objects {
        match obj {
            RenderObject::TetNodes(gm, instances) => color_instance_random(gm, instances),
            RenderObject::TetEdges(gm, instances) => color_instance_random(gm, instances),
            RenderObject::TetBoundarySurface(gm) => (),
            RenderObject::TetSurface(gm) => (),
        }
    }
}

fn color_instance_random(
    gm: &mut three_d::Gm<three_d::InstancedMesh, three_d::ColorMaterial>,
    instances: &mut Instances,
) {
    let mut rng = rand::rng();
    let colors = (0..instances.transformations.len()).map(|_| utils::random_color(&mut rng));
    instances.colors = Some(Vec::from_iter(colors));

    gm.set_instances(instances);
}

fn color_edge_length(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    let (min, max) = tet_graph.calculate_edge_length_bounds();
}

fn color_dist_to_ground(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    todo!()
}
