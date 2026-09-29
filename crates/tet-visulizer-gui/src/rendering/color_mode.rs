use std::fmt::Display;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use three_d::{Instances, Srgba};

use tet_visulizer_core::data::{
    DikstraPath, FaceDirClassification, TetGraph, TetIndex, TetNode, TetVertexId,
    TwoWayNeighborEdge, compute_dist_to_ground,
};

use crate::rendering::{render_object::RenderObject, utils};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ColorMode {
    Random,
    EdgeLength,
    DistToGround,
    FaceDir,
}

impl Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_mode_name = match self {
            ColorMode::Random => "Random",
            ColorMode::EdgeLength => "Edge Length",
            ColorMode::DistToGround => "Dist To Ground",
            ColorMode::FaceDir => "Face Dir",
        };

        f.write_str(color_mode_name)
    }
}

impl ColorMode {
    pub fn color_objects(&self, tet_graph: &TetGraph, objects: &mut [RenderObject]) {
        match self {
            ColorMode::Random => color_random(objects),
            ColorMode::EdgeLength => color_length(tet_graph, objects),
            ColorMode::DistToGround => color_dist_to_ground(tet_graph, objects),
            ColorMode::FaceDir => color_face_dir(tet_graph, objects),
        }
    }
}

fn color_face_dir(tet_graph: &TetGraph, objects: &mut [RenderObject]) {
    for obj in objects {
        match obj {
            RenderObject::TetBoundarySurface(gm) => {
                let new_colors = tet_graph
                    .get_boundary_faces()
                    .iter()
                    .map(|(face, tet)| face.get_dir_classification(&tet_graph, *tet))
                    .map(|dir| match dir {
                        FaceDirClassification::GroundSupported => Srgba::GREEN,
                        FaceDirClassification::Overhang => Srgba::RED,
                        FaceDirClassification::StairStepping => Srgba::BLUE,
                        FaceDirClassification::TopSurface => Srgba::WHITE,
                        FaceDirClassification::Wall => Srgba::BLACK,
                    })
                    .map(|color| color.into())
                    .flat_map(|color| [color, color, color])
                    .collect::<Vec<_>>();

                gm.set_colors(&new_colors).expect("Error Setting Colors");
            }
            _ => (),
        }
    }
}

fn color_random(objects: &mut [RenderObject]) {
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

    color_by_metric(
        objects,
        tet_graph,
        (min, max),
        |_node_index, node| node_avg_edge_length(node),
        |edge| Some(edge.dist()),
    );
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
    let path_map = compute_dist_to_ground(tet_graph);

    let (min, max) =
        path_map
            .iter()
            .fold((f64::MAX, f64::MIN), |(min, max), (_tet_index, path)| {
                let dist: f64 = path.dist().into();

                (min.min(dist), max.max(dist))
            });

    color_by_metric(
        objects,
        tet_graph,
        (min, max),
        |node_index, _node| path_map.get(&node_index).map(|path| path.dist().into()),
        |edge| {
            let a_dist = node_dist(&path_map, edge.neighbor_a());
            let b_dist = node_dist(&path_map, edge.neighbor_b());

            match (a_dist, b_dist) {
                (Some(a_dist), Some(b_dist)) => Some((a_dist + b_dist) / 2.),
                _ => None,
            }
        },
    );
}

fn node_dist(
    path_map: &std::collections::HashMap<TetIndex, DikstraPath>,
    node_index: TetIndex,
) -> Option<f64> {
    path_map.get(&node_index).map(|path| path.dist().into())
}

fn color_by_metric(
    objects: &mut [RenderObject],
    tet_graph: &TetGraph,
    (min, max): (f64, f64),
    node_metric: impl Fn(TetIndex, &TetNode) -> Option<f64>,
    edge_metric: impl Fn(&TwoWayNeighborEdge) -> Option<f64>,
) {
    const FALLBACK_COLOR: Srgba = Srgba::BLACK;
    for object in objects {
        match object {
            RenderObject::TetNodes(gm, instances) => {
                let colors: Vec<Srgba> = tet_graph
                    .get_nodes_iter()
                    .map(|(node_index, node)| node_metric(node_index, node))
                    .map(|avg| {
                        let Some(avg) = avg else {
                            return FALLBACK_COLOR;
                        };

                        let interp = (avg - min) / (max - min);
                        utils::interp_color(interp as f32, Srgba::BLUE, Srgba::RED)
                    })
                    .collect();

                instances.colors = Some(colors);

                gm.set_instances(instances);
            }
            RenderObject::TetEdges(gm, instances) => {
                let edges = tet_graph.get_neighbor_edges();

                instances.colors = Some(
                    edges
                        .iter()
                        .map(|edge| edge_metric(edge))
                        .map(|metric| {
                            let Some(metric) = metric else {
                                return FALLBACK_COLOR;
                            };

                            let interp = (metric - min) / (max - min);
                            utils::interp_color(interp as f32, Srgba::BLUE, Srgba::RED)
                        })
                        .collect(),
                );

                gm.set_instances(instances);
            }
            _ => (),
        }
    }
}
