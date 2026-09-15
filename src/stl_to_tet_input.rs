use std::collections::HashMap;

use tritet::InputDataTetMesh;

use crate::StlFile;

pub fn stl_to_tet_input(stl_file: StlFile) -> InputDataTetMesh {
    let n_tri = stl_file.triangles.len();
    let n_p_max_bound = n_tri / 2 + 2; // Assumes well formed and watertight mesh

    let mut points = Vec::with_capacity(n_p_max_bound);
    let mut facets = Vec::with_capacity(n_tri);
    let mut p_to_index: HashMap<[i64; 3], usize> = HashMap::with_capacity(n_p_max_bound);

    for triangle in &stl_file.triangles {
        let mut facet = Vec::with_capacity(3);
        for p in &triangle.vertices {
            let q = quantize_point(p);

            let index: usize;
            if let Some(i) = p_to_index.get(&q) {
                index = i.clone();
            } else {
                index = points.len();
                p_to_index.insert(q, index);

                let p = p_to_f64(p);
                points.push((0, p[0], p[1], p[2]));
            }

            facet.push(index);
        }

        facets.push((0, facet));
    }

    InputDataTetMesh {
        points,
        facets,
        holes: vec![],
        regions: vec![(0, 0.0, 0.0, 1.0, None)],
    }
}

fn quantize_point(p: &[f32; 3]) -> [i64; 3] {
    const PRECISION: f32 = 1000000.;
    p.map(|f| (f * PRECISION as f32).round() as i64)
}

fn p_to_f64(p: &[f32; 3]) -> [f64; 3] {
    p.map(|f| f as f64)
}
