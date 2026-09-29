use cgmath::{InnerSpace, Vector3};

use crate::data::{TetGraph, TetIndex, tet_graph::tet::TetVertexIndex};

pub enum FaceDirClassification {
    GroundSupported,
    Overhang,
    StairStepping,
    TopSurface,
    Wall,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TetFace {
    pub(in crate::data::tet_graph) points: [TetVertexIndex; 3],
}

impl TetFace {
    pub fn new(a: TetVertexIndex, b: TetVertexIndex, c: TetVertexIndex) -> TetFace {
        let mut points = [a, b, c];
        points.sort();

        Self { points }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, TetVertexIndex> {
        self.points.iter()
    }

    pub fn get_dir_classification(
        &self,
        graph: &TetGraph,
        owned_by: TetIndex,
    ) -> FaceDirClassification {
        let normal = self.calc_normal(graph, owned_by);

        let angle = normal.z.acos().to_degrees();

        match angle {
            _ if angle < 2. => FaceDirClassification::TopSurface,
            _ if angle >= 2. && angle < 27. => FaceDirClassification::StairStepping,
            _ if angle >= 27. && angle < 130. => FaceDirClassification::Wall,

            _ if angle >= 130. => {
                let center = self.calc_center(graph);

                match center.z < 0.1 {
                    true => FaceDirClassification::GroundSupported,
                    false => FaceDirClassification::Overhang,
                }
            }
            _ => FaceDirClassification::GroundSupported,
        }
    }

    pub fn calc_normal(&self, graph: &TetGraph, owned_by: TetIndex) -> cgmath::Vector3<f64> {
        let points = self.points.map(|p| graph.get_point(p));

        let (p1, p2) = (points[1] - points[0], points[2] - points[0]);

        let mut normal = p1.cross(p2).normalize();

        // Flip normal if normal is facing the interior of the tet
        if normal.dot(graph.get_node(owned_by).center - points[0]) > 0.0 {
            normal = -normal;
        }

        normal
    }
    pub fn calc_center(&self, graph: &TetGraph) -> Vector3<f64> {
        let points = self.points.map(|p| graph.get_point(p));

        let center: Vector3<f64> = points
            .iter()
            .fold(Vector3::new(0., 0., 0.), |acc, point| acc + *point)
            / 3.;

        center
    }
}
