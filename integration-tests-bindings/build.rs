// Copyright 2021 Xayn AG
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

use std::{
    env::{self, set_current_dir},
    path::PathBuf,
    process::Command,
};

use cbindgen::{generate_with_config, Config};

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let workspace_path = manifest_dir.parent().unwrap();
    let dart_dir = workspace_path.join("integration_tests");
    let header_out_file = dart_dir.join("include/IntegrationTestsFfi.h");
    let config_file = manifest_dir.join("cbindgen.toml");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-env-changed=DISABLE_AUTO_DART_FFIGEN");
    println!("cargo:rerun-if-changed={}", header_out_file.display());
    println!("cargo:rerun-if-changed={}", config_file.display());

    let config = Config::from_file(config_file).expect("Failed to read config.");
    generate_with_config(manifest_dir, config)
        .expect("Failed to generate bindings (did you use nightly).")
        .write_to_file(header_out_file);

    if is_auto_dart_ffigen_enabled() {
        let ffigen_out_file = dart_dir.join("lib/src/genesis.ffigen.dart");
        println!("cargo:rerun-if-changed={}", ffigen_out_file.display());
        set_current_dir(dart_dir).unwrap();
        run_cmd(Command::new("dart").args(&["pub", "get"]));
        run_cmd(Command::new("dart").args(&["pub", "run", "ffigen", "--config", "ffigen.yaml"]));
    }
}

fn is_auto_dart_ffigen_enabled() -> bool {
    env::var("DISABLE_AUTO_DART_FFIGEN")
        .ok()
        .map_or(true, |v| v.trim() != "1")
}

fn run_cmd(cmd: &mut Command) {
    let es = cmd.status().unwrap();
    if !es.success() {
        panic!("Failed to run command.");
    }
}
