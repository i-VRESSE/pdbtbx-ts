use wasm_bindgen_test::*;
use pdbtbx_ts::*;
use std::collections::HashMap;

#[wasm_bindgen_test]
pub fn test_single_atom() {
    let input = "ATOM      1  N  BALA A   1      72.883  57.697  56.410  0.50 83.80           N
END
    ";

    let result = open_pdb(input).unwrap();

    let result_deserialized:PDBInfo = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = PDBInfo {
        identifier: None,
        chains: vec!["A".to_string()],
        residue_sequence_numbers: vec![1],
        residues_per_chain: HashMap::from([(
            "A".to_string(),
            vec![
                ResidueInfo {
                    number: 1,
                    insertion_code: "-".to_string()
                }
            ]
        )]),
        warnings: vec![]
    };
    assert_eq!(result_deserialized, expected);
}

#[wasm_bindgen_test]
pub fn test_master_err() {
    let input = "ATOM      1  N  BALA A   1      72.883  57.697  56.410  0.50 83.80           N
MASTER        0    0    0    0    0    0    0    0    2    0    0    0
END
    ";

    let result = pdbtbx_ts::open_pdb(input).unwrap_err();

    let result_deserialized: Vec<String> = serde_wasm_bindgen::from_value(result).unwrap();
    let expected =  vec![
        "StrictWarning: MASTER checksum failed\n  ╷\n2 │ MASTER        0    0    0    0    0    0    0    0    2    0    0    0\n  ╵\nThe number of Atoms (1) is different then posed in the MASTER Record (2)\n"
    ];
    assert_eq!(result_deserialized, expected);
}
