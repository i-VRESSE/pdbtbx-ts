use pdbtbx_ts::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn test_single_atom() {
    let input = "ATOM      1  N  BALA A   1      72.883  57.697  56.410  0.50 83.80           N
END
    ";

    let result = open_pdb(input).unwrap();

    let result_deserialized: PDBInfo = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = PDBInfo {
        identifier: None,
        chains: vec!["A".to_string()],
        residue_sequence_numbers: vec![1],
        residues_per_chain: HashMap::from([(
            "A".to_string(),
            vec![ResidueInfo {
                number: 1,
                insertion_code: "-".to_string(),
            }],
        )]),
        warnings: vec![],
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

#[wasm_bindgen_test]
pub fn test_dup_resnum() {
    let input = "ATOM      1  N   THR A  19      51.382  49.015  12.266  1.00  0.15      A
ATOM   2587  N   ALA B  19      27.171  30.284  19.911  1.00  0.14      B
END
    ";

    let result = open_pdb(input).unwrap();

    let result_deserialized: PDBInfo = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = PDBInfo {
        identifier: None,
        chains: vec!["A".to_string(), "B".to_string()],
        residue_sequence_numbers: vec![19],
        residues_per_chain: HashMap::from([
            (
                "A".to_string(),
                vec![ResidueInfo {
                    number: 19,
                    insertion_code: "-".to_string(),
                }],
            ),
            (
                "B".to_string(),
                vec![ResidueInfo {
                    number: 19,
                    insertion_code: "-".to_string(),
                }],
            ),
        ]),
        warnings: vec![],
    };
    assert_eq!(result_deserialized, expected);
}

#[wasm_bindgen_test]
pub fn test_sorted_resnum() {
    let input = "ATOM    588 1HG  GLU A  18     -13.363  -4.163  -2.372  1.00  0.00           H
ATOM      5  CB BALA B   1      74.804  56.369  55.453  0.50 84.29           C
ATOM      6  N  BASP B   2      75.872  56.703  57.905  0.50 85.59           N
END
    ";

    let result = open_pdb(input).unwrap();

    let result_deserialized: PDBInfo = serde_wasm_bindgen::from_value(result).unwrap();
    let expected = PDBInfo {
        identifier: None,
        chains: vec!["A".to_string(), "B".to_string()],
        residue_sequence_numbers: vec![1, 2, 18],
        residues_per_chain: HashMap::from([
            (
                "A".to_string(),
                vec![ResidueInfo {
                    number: 18,
                    insertion_code: "-".to_string(),
                }],
            ),
            (
                "B".to_string(),
                vec![
                    ResidueInfo {
                        number: 1,
                        insertion_code: "-".to_string(),
                    },
                    ResidueInfo {
                        number: 2,
                        insertion_code: "-".to_string(),
                    },
                ],
            ),
        ]),
        warnings: vec![],
    };
    assert_eq!(result_deserialized, expected);
}
