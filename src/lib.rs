mod utils;

use pdbtbx::*;
use std::io::BufReader;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct PDBInfo {
    pub chains: String, // TODO replace comma separated string with Vec<String>
    pub residue_sequence_numbers: Vec<isize>,
}

#[wasm_bindgen]
pub fn open_pdb(content: &str) -> Result<PDBInfo, String> {
    let (pdb, _errors) = open_pdb_raw(
        BufReader::new(content.as_bytes()),
        Context::None,
        StrictnessLevel::Loose,
    )
    .map_err(|e| {
        e.into_iter()
            .fold("".to_string(), |acc, err| acc + &err.to_string() + "\n")
    })?;
    let chains: Vec<String> = pdb.chains().map(Chain::id).map(String::from).collect();
    let residue_sequence_numbers = pdb.residues().map(Residue::serial_number).collect();
    Ok(PDBInfo {
        chains: chains.join(","),
        residue_sequence_numbers,
    })
}
