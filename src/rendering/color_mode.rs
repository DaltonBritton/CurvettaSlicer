use std::fmt::{Display, Write};

use rand::rng;
use strum::IntoEnumIterator;
use three_d::{Instances, Srgba};

use crate::{
    data::{TetGraph, TetNode, TetVertexId},
    rendering::{render_object::RenderObject, utils},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            ColorMode::EdgeLength => color_length(tet_graph, objects),
            ColorMode::DistToGround => color_dist_to_ground(tet_graph, objects),
        }
    }
}

fn color_random(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    for obj in objects {
        match obj {
            RenderObject::TetNodes(gm, instances) => color_instance_random(gm, instances),
            RenderObject::TetEdges(gm, instances) => color_instance_random(gm, instances),
            _ => (),
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

fn color_length(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    let (min, max) = tet_graph.calculate_edge_length_bounds();
    let edges = tet_graph.get_neighbor_edges();

    for object in objects {
        match object {
            RenderObject::TetNodes(gm, instances) => {
                let colors: Vec<Srgba> = tet_graph
                    .get_nodes()
                    .iter()
                    .map(|node| node_avg_edge_length(node))
                    .map(|avg| {
                        let Some(avg) = avg else {
                            return Srgba::BLACK;
                        };

                        let interp = (avg - min) / (max - min);

                        utils::interp_color(interp as f32, Srgba::BLUE, Srgba::RED)
                    })
                    .collect();

                instances.colors = Some(colors);

                gm.set_instances(instances);
            }
            RenderObject::TetEdges(gm, instances) => {
                instances.colors = Some(
                    edges
                        .iter()
                        .map(|edge| (edge.dist() - min) / (max - min))
                        .map(|interp| utils::interp_color(interp as f32, Srgba::BLUE, Srgba::RED))
                        .collect(),
                );

                gm.set_instances(instances);
            }
            _ => (),
        }
    }
}

fn node_avg_edge_length(node: &TetNode) -> Option<f64> {
    let dists: Vec<f64> = TetVertexId::iter()
        .filter_map(|id| node.get_neighbor(id))
        .map(|edge| edge.dist())
        .collect();

    if dists.is_empty() {
        None // boundary tet with zero neighbors — decide a fallback color
    } else {
        Some(dists.iter().sum::<f64>() / dists.len() as f64)
    }
}

fn color_dist_to_ground(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    todo!()
}
