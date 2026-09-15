use std::fs::File;

use binrw::BinRead;
use three_d::context::NONE;
use tritet::{InputDataTetMesh, StrError, Tetgen};

use crate::{StlFile, stl_to_tet_input};

const SAVE_FIGURE: bool = false;

pub fn gen_tet(stl_file: StlFile) -> Result<Tetgen, StrError> {
    let input_data = stl_to_tet_input(stl_file);

    // allocate data for 4 points
    // let input_data = InputDataTetMesh {
    //     points: vec![
    //         (0, 0.0, 5.0, 0.0), // marker, x, y, z
    //         (0, 0.0, 0.0, 0.0),
    //         (0, 5.0, 5.0, 0.0),
    //         (0, 0.0, 5.0, 5.0),
    //     ],
    //     facets: vec![
    //         (0, vec![0, 2, 1]), // marker, point indices
    //         (0, vec![0, 1, 3]),
    //         (0, vec![0, 3, 2]),
    //         (0, vec![1, 2, 3]),
    //     ],
    //     holes: vec![],                           // no holes
    //     regions: vec![(1, 0.1, 0.9, 0.1, None)], // region marker, x, y, z, max volume
    // };

    // allocate generator from input data
    let tetgen = Tetgen::from_input_data(&input_data)?;

    // generate mesh
    let global_max_volume = Some(10.);
    let min_angle = Some(50.);
    tetgen.generate_mesh(true, false, global_max_volume, min_angle)?;

    // draw edges of tetrahedra
    if SAVE_FIGURE {
        let mut file_path = std::env::current_dir().unwrap();
        file_path.push("mesh.vtu");

        tetgen.write_vtu(file_path.as_os_str()).unwrap();
    }

    Ok(tetgen)
}
