mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct PDBInfo {
    pub chains: String, // TODO use Vec<String>
    pub residue_sequence_numbers: Vec<i32>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PDBError;

#[wasm_bindgen]
pub fn open_pdb(_content: &str) -> Result<PDBInfo, PDBError>{
    let mut chains = Vec::new();
    chains.push(String::from("A"));
    let residue_sequence_numbers = vec![1,2,3,4];
    let info = PDBInfo { 
        chains: chains.join(","), 
        residue_sequence_numbers: residue_sequence_numbers
    };
    Ok(info)
}
