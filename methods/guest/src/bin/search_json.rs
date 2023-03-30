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

#![no_main]

use json::parse;
use json_core::Outputs;
use risc0_zkvm::guest::{env, sha};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data1: String = env::read();
    let data2: String = env::read();

    let sha1 = sha::digest(&data1.as_bytes());
    let sha2 = sha::digest(&data2.as_bytes());

    let data1 = parse(&data1).unwrap();
    let data2 = parse(&data2).unwrap();

    let proven_val1 = data1["critical_data"].as_u32().unwrap();
    let proven_val2 = data2["critical_data"].as_u32().unwrap();

    if proven_val1 != proven_val2 {
        panic!("Values of critical_data do not agree!");
    }

    let out = Outputs {
        hash1: *sha1,
        hash2: *sha2,
    };

    env::commit(&out);
}
