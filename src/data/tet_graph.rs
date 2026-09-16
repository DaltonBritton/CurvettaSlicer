use std::{
    array,
    collections::{HashMap, hash_map},
    fmt::Display,
};

use three_d::{MetricSpace, Vector3};

use crate::data::tet::{self, Tet};

#[derive(Debug, Clone, Copy)]
pub struct NeighborEdge {
    _index: usize,
    _dist: f64,
}

#[derive(Debug)]
pub struct TetNode {
    tet: Tet,
    neighbors: [Option<NeighborEdge>; 4],
    center: Vector3<f64>,
}

impl TetNode {
    fn new(point_indices: [usize; 4], points: &[Vector3<f64>]) -> Self {
        let tet = Tet::new(point_indices);
        let center = Self::calculate_center(point_indices, points);

        Self {
            tet,
            neighbors: [Option::None; 4],
            center,
        }
    }

    fn calculate_center(point_indices: [usize; 4], points: &[Vector3<f64>]) -> Vector3<f64> {
        let sum = point_indices
            .iter()
            .map(|i| points[*i])
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
    boundary_faces: HashMap<[usize; 3], usize>,
}

impl TetGraph {
    pub fn new(tetgen: &tritet::Tetgen) -> Self {
        println!("Computing Graph");
        let points = Self::read_vertices(&tetgen);
        let mut nodes = Self::read_tets(&tetgen, &points);
        let boundary_faces = Self::assosiate_neighbors(&mut nodes);
        println!("Graph Complete");

        Self {
            points: points,
            nodes,
            boundary_faces: boundary_faces,
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
            let point_indices: [usize; 4] = array::from_fn(|p_i| tetgen.out_cell_point(tet_i, p_i));

            let node = TetNode::new(point_indices, points);
            nodes.push(node);
        }

        println!("Read {} Tets!", nodes.len());

        nodes
    }

    fn assosiate_neighbors(nodes: &mut Vec<TetNode>) -> HashMap<[usize; 3], usize> {
        let mut boundary_faces = HashMap::with_capacity(nodes.len());
        let mut num_neighbors: u64 = 0;

        for node_i in 0..nodes.len() {
            for face_i in 0..4 {
                let face = Self::get_face_opposing_point(nodes.get(node_i).unwrap(), face_i);

                let boundary_face = boundary_faces.entry(face);
                match boundary_face {
                    hash_map::Entry::Occupied(neighbor_node_entry) => {
                        let neighbor_i = *neighbor_node_entry.get();

                        let [node, neighbor] = nodes.get_disjoint_mut([node_i, neighbor_i]).expect(
                            "Neighbor_i should always exist in nodes before associating neighbors",
                        );

                        let dist = node.center.distance(neighbor.center);

                        node.neighbors[face_i] = Some(NeighborEdge {
                            _index: neighbor_i,
                            _dist: dist,
                        });

                        neighbor.neighbors[face_i] = Some(NeighborEdge {
                            // TODO: Need to find correct face id
                            _index: node_i,
                            _dist: dist,
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

        boundary_faces
    }

    fn get_face_opposing_point(node: &TetNode, point: usize) -> [usize; 3] {
        let mut face_p_iter = node
            .tet
            .points
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != point);
        let mut face: [usize; 3] = std::array::from_fn(|_| *face_p_iter.next().unwrap().1);
        face.sort(); // TODO: Make Face Struct??

        face
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_points(&self) -> &[Vector3<f64>] {
        &self.points
    }

    pub fn get_nodes(&self) -> &[TetNode] {
        &self.nodes
    }

    pub fn get_boundary_faces(&self) -> &HashMap<[usize; 3], usize> {
        &self.boundary_faces
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
            TetNode::new([0, 1, 2, 3], &points),
            TetNode::new([1, 2, 3, 4], &points),
        ];

        let remaining_faces = TetGraph::assosiate_neighbors(&mut nodes);

        assert!(nodes[0].neighbors[0].is_some_and(|neighbor| neighbor._index == 1));
        assert!(nodes[1].neighbors[3].is_some_and(|neighbor| neighbor._index == 0));

        assert_eq!(remaining_faces.len(), 6);
    }
}
