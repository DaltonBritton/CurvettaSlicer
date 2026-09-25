use std::{error::Error, fs, path::Path, process::Command};

use mshio::{ElementType, MshFile};

pub fn gen_tet(
    input_path: &Path,
    output_path: &Path,
    f_wild_tet_bin: &Path,
) -> Result<(), Box<dyn Error>> {
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

    let msh = read_msh(output_path)?;

    msh_to_tet_graph(msh)?;
    todo!()
}

fn msh_to_tet_graph(msh: MshFile<u64, i32, f64>) -> Result<(), Box<dyn Error>> {
    let data = msh.data;
    let points = data.nodes.ok_or("Msh contains no nodes")?;
    let elements = data.elements.ok_or("Msh contains no elements")?;
    let tets = elements
        .element_blocks
        .iter()
        .filter(|block| block.element_type == ElementType::Tet4)
        .for_each(|block| println!("{}", block.elements.len()));
    Ok(())
}

fn read_msh(msh_path: &Path) -> Result<MshFile<u64, i32, f64>, Box<dyn Error>> {
    let msh_bytes = fs::read(msh_path)?;
    let parser_result = mshio::parse_msh_bytes(msh_bytes.as_slice());
    let msh = parser_result.map_err(|e| format!("Error while parsing msh file:\n{}", e))?;
    Ok(msh)
}
