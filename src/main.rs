use binrw::BinRead;
use std::{env, fs::File, time::Instant};
use three_d::*;

use tet_visulizer::{data::TetGraph, *};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("File Path not provided");

    let mut file = File::open(filename).expect("Unable to Open File");
    let stl_file = StlFile::read(&mut file).expect("Unable to Parse Stl File");

    let tet_mesh = gen_tet(stl_file).expect("Error Occured while generating tets");

    let tet_graph = TetGraph::new(&tet_mesh);

    App::new(tet_graph).run();
}
