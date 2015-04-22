// Copyright 2015 The Athena Developers.
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

use std::error::Error;
use std::path::PathBuf;
use docopt::Docopt;

use zeus::project::ZeusProject;

static USAGE: &'static str = "
Athena's project build system.

Usage:
    zeus new <path>
";

#[derive(RustcDecodable, Debug)]
struct Flags {
    arg_path: String
}

pub fn execute() -> Result<(), Box<Error>> {
    // Parse in the command line flags
    let flags: Flags = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // Create a new project
    let path = PathBuf::from(flags.arg_path);
    try!(ZeusProject::create(path));

    return Ok(());
}
