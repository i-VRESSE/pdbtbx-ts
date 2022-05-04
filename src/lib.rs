mod utils;

use pdbtbx::*;
use std::io::BufReader;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::{Serialize};

#[derive(Serialize)]
pub struct PDBInfo {
    pub chains: Vec<String>,
    pub residue_sequence_numbers: Vec<isize>,
}

// TODO instead of writing open_pdb typescript signature manually try to use https://crates.io/crates/typescript-definitions
#[wasm_bindgen(typescript_custom_section)]
const TS_CUSTOM_SECTION: &'static str = r#"
interface IPDBInfo {
    chains: string[]
    residue_sequence_numbers: number[]
}

export function open_pdb (content: string): IPDBInfo
"#;

#[wasm_bindgen(skip_typescript)]
pub fn open_pdb(content: &str) -> Result<JsValue, String> {
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
    let info = PDBInfo {
        chains,
        residue_sequence_numbers,
    };
    let js_value = serde_wasm_bindgen::to_value(&info).unwrap();
    Ok(js_value)
}
