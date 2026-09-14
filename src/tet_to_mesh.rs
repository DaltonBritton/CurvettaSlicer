use std::u8;

use rand::{RngExt, rngs::ThreadRng};
use three_d::{CpuMesh, Srgba, Vector3};
use tritet::Tetgen;

pub fn tet_to_mesh(tetgen: Tetgen) -> CpuMesh {
    let n_tets = tetgen.out_ncell();
    let n_points = tetgen.out_npoint();

    let points: Vec<Vector3<f64>> = (0..n_points).map(|i| read_point(&tetgen, i)).collect();

    // Each triangle gets its own copy of its 3 vertices (instead of sharing
    // vertices with neighboring triangles) so that per-vertex colors can be
    // used to give every triangle a distinct, flat color.
    let mut positions: Vec<Vector3<f64>> = Vec::with_capacity(n_tets * 4 * 3);
    let mut triangles: Vec<u32> = Vec::with_capacity(n_tets * 4 * 3);
    let mut colors: Vec<Srgba> = Vec::with_capacity(n_tets * 4 * 3);

    //used for the color generation
    let mut rng = rand::rng();

    for i in 0..n_tets {
        let mut tet_points: [u32; 4] = [0; 4];

        for p in 0..4 {
            tet_points[p] = tetgen.out_cell_point(i, p) as u32;
        }

        const PERMUTATIONS: [[usize; 3]; 4] = [[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];

        for perm in PERMUTATIONS {
            let color = random_color(&mut rng);

            for vertex_i in perm {
                let point = points[tet_points[vertex_i] as usize];
                let new_index = positions.len() as u32;

                positions.push(point);
                colors.push(color);
                triangles.push(new_index);
            }
        }
    }

    CpuMesh {
        positions: three_d::Positions::F64(positions),
        indices: three_d::Indices::U32(triangles),
        colors: Some(colors),
        ..Default::default()
    }
}

fn read_point(tetgen: &Tetgen, index: usize) -> Vector3<f64> {
    let x = tetgen.out_point(index, 0);
    let y = tetgen.out_point(index, 1);
    let z = tetgen.out_point(index, 2);

    Vector3 { x, y, z }
}

fn random_color(rng: &mut ThreadRng) -> Srgba {
    Srgba {
        r: rng.random_range(0..255),
        g: rng.random_range(0..255),
        b: rng.random_range(0..255),
        a: u8::MAX,
    }
}
