use binrw::BinRead;
use std::{env, fs::File, path::Path};

use tet_visulizer_core::{StlFile, data::TetGraph, gen_tet};
use tet_visulizer_gui::App;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filepath = Path::new(args.get(1).expect("File Path not provided"));
    let output_filepath = Path::new(args.get(2).expect("File Path not provided"));
    let f_tet_wild_bin = std::env::var("fTetWild_bin").expect("Unable to locate fTetWild");
    let f_tet_wild_bin = Path::new(&f_tet_wild_bin);

    let tet_graph = gen_tet(input_filepath, output_filepath, f_tet_wild_bin)
        .expect("Error Occured while generating tets");

    App::new(tet_graph).run();
}
