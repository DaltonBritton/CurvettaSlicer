use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap},
};

use ordered_float::OrderedFloat;
use strum::IntoEnumIterator;

use crate::data::{TetGraph, TetIndex, TetVertexId};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DikstraPath {
    index: TetIndex,
    dist: OrderedFloat<f64>,
    prev: Option<TetIndex>,
}

impl DikstraPath {
    pub fn dist(&self) -> OrderedFloat<f64> {
        self.dist
    }
}

impl Ord for DikstraPath {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for DikstraPath {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.dist.cmp(&other.dist))
    }
}

pub fn compute_dist_to_ground(tet_graph: &TetGraph) -> HashMap<TetIndex, DikstraPath> {
    let n_nodes = tet_graph.len();

    let mut dist_estimates = BinaryHeap::with_capacity(n_nodes);
    let mut dists: HashMap<TetIndex, DikstraPath> = HashMap::with_capacity(n_nodes);

    // Init starting node dists
    for (tet_index, node) in tet_graph.get_nodes_iter() {
        let tet = node.tet();

        let is_supported = TetVertexId::iter()
            .map(|vertex_id| tet.get_point_index(vertex_id))
            .map(|point_id| tet_graph.get_point(point_id))
            .any(|point| point.z < 0.01);

        if is_supported {
            let path = DikstraPath {
                index: tet_index,
                dist: OrderedFloat(0.),
                prev: None,
            };

            dists.insert(tet_index, path.clone());
            dist_estimates.push(Reverse(path));
        }
    }

    while let Some(Reverse(src_path)) = dist_estimates.pop() {
        if dist_estimates.len() % 1000 == 0 {
            println!("dist_estimates remaining: {}", dist_estimates.len());
        }
        // Skip if a faster path has been processed
        let node = tet_graph.get_node(src_path.index);

        if let Some(existing_path) = dists.get(&src_path.index) {
            if existing_path.dist < src_path.dist {
                continue;
            }
        }

        for vertex_id in TetVertexId::iter() {
            let Some(neighbor) = node.get_neighbor(vertex_id) else {
                // skip if no neighbor exists across from vertex_id
                continue;
            };

            let neighbor_index = neighbor.index();
            let neighbor_dist = src_path.dist + neighbor.dist();

            let neighbor_path = DikstraPath {
                index: neighbor_index,
                dist: neighbor_dist,
                prev: Some(src_path.index),
            };

            let dists_entry = dists.entry(neighbor_index);
            match dists_entry {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    let existing_path = occupied_entry.get();
                    if existing_path.dist > neighbor_dist {
                        dist_estimates.push(Reverse(neighbor_path.clone()));
                        occupied_entry.insert(neighbor_path);
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    dist_estimates.push(Reverse(neighbor_path.clone()));
                    vacant_entry.insert(neighbor_path);
                }
            }
        }
    }

    dists
}
