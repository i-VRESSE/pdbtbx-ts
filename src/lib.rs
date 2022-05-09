mod utils;

use pdbtbx::*;
use std::collections::HashMap;
use std::io::BufReader;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResidueInfo {
    pub number: isize,
    pub insertion_code: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PDBInfo {
    pub identifier: Option<String>,
    pub chains: Vec<String>,
    pub residue_sequence_numbers: Vec<isize>,
    // TODO serialize to js Object instead of Map (https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
    pub residues_per_chain: HashMap<String, Vec<ResidueInfo>>,
    pub warnings: Vec<String>,
}

// TODO instead of writing open_pdb typescript signature manually try to use https://crates.io/crates/typescript-definitions
#[wasm_bindgen(typescript_custom_section)]
const TS_CUSTOM_SECTION: &'static str = r#"

interface ResidueInfo {
    number: number
    insertion_code: String
}

interface PDBInfo {
    identifier?: string
    chains: string[]
    residue_sequence_numbers: number[]
    residues_per_chain: Map<string, ResidueInfo[]>
    warnings: string[]
}

export function open_pdb (content: string): PDBInfo
"#;

fn pdb2pdbinfo(pdb: PDB, warnings: Vec<PDBError>) -> PDBInfo {
    let chains: Vec<String> = pdb.chains().map(Chain::id).map(String::from).collect();
    let residue_sequence_numbers = pdb.residues().map(Residue::serial_number).collect();
    let mut residues_per_chain = HashMap::new();
    for chain in pdb.chains() {
        let residues_of_chain = chain
            .residues()
            .map(|r| ResidueInfo {
                number: r.serial_number(),
                insertion_code: r.insertion_code().unwrap_or("-").to_string(),
            })
            .collect();
        residues_per_chain.insert(String::from(chain.id()), residues_of_chain);
    }
    let mut warnings_as_strings = Vec::new();
    for warning in warnings {
        warnings_as_strings.push(format!("{:?}", warning))
    }
    return PDBInfo {
        identifier: pdb.identifier,
        chains,
        residue_sequence_numbers,
        residues_per_chain,
        warnings: warnings_as_strings,
    };
}

#[wasm_bindgen(skip_typescript)]
pub fn open_pdb(content: &str) -> Result<JsValue, JsValue> {
    match open_pdb_raw(
        BufReader::new(content.as_bytes()),
        Context::None,
        StrictnessLevel::Loose,
    ) {
        Ok((pdb, warnings)) => {
            let info = pdb2pdbinfo(pdb, warnings);
            let js_value = serde_wasm_bindgen::to_value(&info).unwrap();
            Ok(js_value)
        }
        Err(errors) => {
            let mut errors_as_strings = Vec::new();
            for error in errors {
                errors_as_strings.push(format!("{:?}", error))
            }
            let js_value = serde_wasm_bindgen::to_value(&errors_as_strings).unwrap();
            Err(js_value)
        }
    }
}
