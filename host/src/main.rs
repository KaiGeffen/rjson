// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::prelude::*;

use json_core::Outputs;
use methods::{SEARCH_JSON_ELF, SEARCH_JSON_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::Prover;

fn main() {
    // Load data 1
    let mut file1 =
        std::fs::File::open("res/example1.json").expect("Example file should be accessible");
    let mut data1 = String::new();
    file.read_to_string(&mut data1)
        .expect("Should not have I/O errors");

    // Load data 2
    let mut file2 =
        std::fs::File::open("res/example2.json").expect("Example file should be accessible");
    let mut data2 = String::new();
    file2.read_to_string(&mut data2)
        .expect("Should not have I/O errors");

    // Make the prover.
    let mut prover = Prover::new(SEARCH_JSON_ELF, SEARCH_JSON_ID)
        .expect("Prover should be constructed from matching method code & ID");

    // Add both json data
    prover.add_input_u32_slice(&to_vec(&data1).expect("should be serializable"));
    prover.add_input_u32_slice(&to_vec(&data2).expect("should be serializable"));

    // Run prover & generate receipt
    let receipt = prover.run().expect("Code should be provable");

    receipt
        .verify(SEARCH_JSON_ID)
        .expect("Proven code should verify");

    let journal = &receipt.journal;
    let outputs: Outputs = from_slice(&journal).expect("Journal should contain an Outputs object");

    println!("\nThe following JSON files agree on the value of 'critical_data':\n{}\n{}\n", outputs.hash1, outputs.hash2);
}
