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
    pub chains: String, // TODO replace comma seperated string with Vec<String>
    pub residue_sequence_numbers: Vec<isize>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PDBError;

#[wasm_bindgen]
pub fn open_pdb(content: &str) -> Result<PDBInfo, PDBError>{
    let (pdb, _errors) =  open_pdb_raw(BufReader::new(content.as_bytes()), Context::None, StrictnessLevel::Loose).unwrap();
    let chains: Vec<String> = pdb.chains().map(Chain::id).map(String::from).collect();
    let residue_sequence_numbers: Vec<isize> = pdb.residues().map(Residue::serial_number).collect();
    let info = PDBInfo { 
        chains: chains.join(","), 
        residue_sequence_numbers: residue_sequence_numbers
    };
    Ok(info)
}
