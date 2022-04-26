mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PDBInfo {
    chains: String,
    residue_sequence_numbers: String,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct PDBError;

#[wasm_bindgen]
pub fn open_pdb(_content: &str) -> Result<PDBInfo, PDBError>{
    let info = PDBInfo { chains: String::from("X"), residue_sequence_numbers: String::from("Y") };
    Ok(info)
}
