use std::{error::Error, fs, path::Path, process::Command};

use mshio::{ElementType, MshFile};
use nom::Parser;

use crate::{data::TetGraph, msh_parser};

pub fn gen_tet(
    input_path: &Path,
    output_path: &Path,
    f_wild_tet_bin: &Path,
) -> Result<TetGraph, Box<dyn Error>> {
    let mut tet_wild_cmd = Command::new(f_wild_tet_bin);

    tet_wild_cmd
        .arg("-i")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .arg("--no-binary"); // TODO: calc actual epsr ie. espr = resolution / bbox;

    tet_wild_cmd
        .output()
        .map_err(|_| "TetWild exited unexpectedly")?;

    let graph = read_msh(output_path)?;

    Ok(graph)
}

fn read_msh(msh_path: &Path) -> Result<TetGraph, Box<dyn Error>> {
    let msh_bytes = fs::read(msh_path)?;

    let parser_result = msh_parser::parser::<nom::error::Error<&[u8]>>().parse_complete(&msh_bytes);
    let (_, graph) = parser_result.map_err(|_e| "Failed To Parse msh file")?;
    Ok(graph)
}
