use std::{
    array,
    collections::{HashMap, hash_map},
    fmt::Display,
};

use cgmath::{MetricSpace, Vector3};
use strum::IntoEnumIterator;

mod tet;

pub use tet::*;

#[derive(Debug, Clone, Copy)]
pub struct NeighborEdge {
    index: TetIndex,
    dist: f64,
}

impl NeighborEdge {
    pub fn index(&self) -> TetIndex {
        self.index
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TwoWayNeighborEdge {
    neighbor_a: TetIndex,
    neighbor_b: TetIndex,
    dist: f64,
}

impl TwoWayNeighborEdge {
    pub fn neighbor_a(&self) -> TetIndex {
        self.neighbor_a
    }

    pub fn neighbor_b(&self) -> TetIndex {
        self.neighbor_b
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }
}

impl NeighborEdge {}

#[derive(Debug)]
pub struct TetNode {
    tet: Tet,
    neighbors: [Option<NeighborEdge>; 4],
    center: Vector3<f64>,
}

impl TetNode {
    fn new(point_indices: [TetVertexIndex; 4], points: &[Vector3<f64>]) -> Self {
        let tet = Tet::new(point_indices);
        let center = Self::calculate_center(point_indices, points);

        Self {
            tet,
            neighbors: [Option::None; 4],
            center,
        }
    }

    fn calculate_center(
        point_indices: [TetVertexIndex; 4],
        points: &[Vector3<f64>],
    ) -> Vector3<f64> {
        let sum = point_indices
            .iter()
            .map(|i| points[i.index])
            .reduce(|acc, p| acc + p)
            .unwrap_or(Vector3 {
                x: 0.,
                y: 0.,
                z: 0.,
            });

        sum / 4.
    }

    pub fn tet(&self) -> &Tet {
        &self.tet
    }

    pub fn center(&self) -> Vector3<f64> {
        self.center
    }

    pub fn get_neighbor(&self, face_id: TetVertexId) -> Option<NeighborEdge> {
        match face_id {
            TetVertexId::A => self.neighbors[0],
            TetVertexId::B => self.neighbors[1],
            TetVertexId::C => self.neighbors[2],
            TetVertexId::D => self.neighbors[3],
        }
    }

    fn set_neighbor(&mut self, face_id: TetVertexId, neighbor: Option<NeighborEdge>) {
        match face_id {
            TetVertexId::A => self.neighbors[0] = neighbor,
            TetVertexId::B => self.neighbors[1] = neighbor,
            TetVertexId::C => self.neighbors[2] = neighbor,
            TetVertexId::D => self.neighbors[3] = neighbor,
        }
    }
}

impl Display for TetNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TetNode {{\n
            \ttet: {},\n
            \tneighbors: {:?}\n}}",
            self.tet, self.neighbors
        )
    }
}

pub struct TetGraph {
    points: Vec<Vector3<f64>>,
    nodes: Vec<TetNode>,
    boundary_faces: HashMap<TetFace, TetIndex>,
    neighbor_edges: Vec<TwoWayNeighborEdge>,
}

impl TetGraph {
    pub fn new(tetgen: &tritet::Tetgen) -> Self {
        println!("Computing Graph");
        let points = Self::read_vertices(&tetgen);
        let mut nodes = Self::read_tets(&tetgen, &points);
        let (boundary_faces, neighbor_edges) = Self::assosiate_neighbors(&mut nodes);
        println!("Graph Complete");

        Self {
            points: points,
            nodes,
            boundary_faces: boundary_faces,
            neighbor_edges: neighbor_edges,
        }
    }

    fn read_vertices(tetgen: &tritet::Tetgen) -> Vec<Vector3<f64>> {
        let n = tetgen.out_npoint();

        let mut points = Vec::with_capacity(n);

        for i in 0..n {
            let point = Vector3::new(
                tetgen.out_point(i, 0),
                tetgen.out_point(i, 1),
                tetgen.out_point(i, 2),
            );

            points.push(point);
        }

        println!("Read {} Points!", points.len());

        points
    }

    fn read_tets(tetgen: &tritet::Tetgen, points: &Vec<Vector3<f64>>) -> Vec<TetNode> {
        let n = tetgen.out_ncell();
        let mut nodes = Vec::with_capacity(n);

        for tet_i in 0..n {
            let point_indices: [TetVertexIndex; 4] =
                array::from_fn(|p_i| TetVertexIndex::new(tetgen.out_cell_point(tet_i, p_i)));

            let node = TetNode::new(point_indices, points);
            nodes.push(node);
        }

        println!("Read {} Tets!", nodes.len());

        nodes
    }

    fn assosiate_neighbors(
        nodes: &mut Vec<TetNode>,
    ) -> (HashMap<TetFace, TetIndex>, Vec<TwoWayNeighborEdge>) {
        let mut boundary_faces: HashMap<TetFace, TetIndex> = HashMap::new();
        let mut edges: Vec<TwoWayNeighborEdge> = Vec::with_capacity(nodes.len() * 3);
        let mut num_neighbors: u64 = 0;

        for node_i in 0..nodes.len() {
            let node_i = TetIndex::new(node_i);
            for face_id in TetVertexId::iter() {
                let face: TetFace = nodes
                    .get(node_i.index)
                    .unwrap()
                    .tet()
                    .get_face_across_from_vertex(face_id);

                let boundary_face = boundary_faces.entry(face);
                match boundary_face {
                    hash_map::Entry::Occupied(neighbor_node_entry) => {
                        let neighbor_i: TetIndex = *neighbor_node_entry.get();

                        let [node, neighbor] = nodes.get_disjoint_mut([node_i.index, neighbor_i.index]).expect(
                            "Neighbor_i should always exist in nodes before associating neighbors",
                        );

                        let dist = node.center.distance(neighbor.center);

                        node.set_neighbor(
                            face_id,
                            Some(NeighborEdge {
                                index: neighbor_i,
                                dist: dist,
                            }),
                        );

                        let neighbor_face_id = neighbor.tet()
                            .get_vertex_across_from_face(face)
                            .expect("neighbor must always contain their neighbors face.
                                However while trying to associate neighbors no such face was found.");

                        neighbor.set_neighbor(
                            neighbor_face_id,
                            Some(NeighborEdge {
                                index: node_i,
                                dist: dist,
                            }),
                        );

                        edges.push(TwoWayNeighborEdge {
                            neighbor_a: node_i,
                            neighbor_b: neighbor_i,
                            dist,
                        });

                        neighbor_node_entry.remove();
                        num_neighbors += 1;
                    }
                    hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(node_i);
                    }
                }
            }
        }

        println!("Found {} Neighbors!", num_neighbors);
        println!("{} Remaining Boundary Faces!", boundary_faces.len());

        (boundary_faces, edges)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_point(&self, TetVertexIndex { index }: TetVertexIndex) -> Vector3<f64> {
        self.points[index]
    }

    pub fn get_points(&self) -> &[Vector3<f64>] {
        &self.points
    }

    pub fn get_node(&self, TetIndex { index }: TetIndex) -> &TetNode {
        &self.nodes[index]
    }

    pub fn get_nodes(&self) -> &[TetNode] {
        &self.nodes
    }

    pub fn get_nodes_iter<'a>(
        &'a self,
    ) -> std::iter::Map<
        std::iter::Enumerate<std::slice::Iter<'a, TetNode>>,
        impl FnMut((usize, &'a TetNode)) -> (TetIndex, &'a TetNode),
    > {
        self.nodes
            .iter()
            .enumerate()
            .map(|(i, node)| (TetIndex::new(i), node))
    }

    pub fn get_boundary_faces(&self) -> &HashMap<TetFace, TetIndex> {
        &self.boundary_faces
    }

    pub fn get_neighbor_edges(&self) -> &[TwoWayNeighborEdge] {
        &self.neighbor_edges
    }

    pub fn calculate_edge_length_bounds(&self) -> (f64, f64) {
        let (mut min, mut max) = (f64::MAX, f64::MIN);
        for edge in &self.neighbor_edges {
            min = min.min(edge.dist);
            max = max.max(edge.dist);
        }

        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use three_d::Vector3;

    use super::*;

    #[test]
    fn a() {
        let points = [
            Vector3::new(0., 0., 0.),
            Vector3::new(0., 0., 1.),
            Vector3::new(0., 1., 0.),
            Vector3::new(1., 0., 0.),
            Vector3::new(1., 1., 1.),
        ];

        let mut nodes = vec![
            TetNode::new(
                [
                    TetVertexIndex::new(0),
                    TetVertexIndex::new(1),
                    TetVertexIndex::new(2),
                    TetVertexIndex::new(3),
                ],
                &points,
            ),
            TetNode::new(
                [
                    TetVertexIndex::new(1),
                    TetVertexIndex::new(2),
                    TetVertexIndex::new(3),
                    TetVertexIndex::new(4),
                ],
                &points,
            ),
        ];

        let (remaining_faces, _edges) = TetGraph::assosiate_neighbors(&mut nodes);

        assert!(nodes[0].neighbors[0].is_some_and(|neighbor| neighbor.index == TetIndex::new(1)));
        assert!(nodes[1].neighbors[3].is_some_and(|neighbor| neighbor.index == TetIndex::new(0)));

        assert_eq!(remaining_faces.len(), 6);
    }
}
