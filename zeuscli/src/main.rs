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

extern crate zeus;
extern crate rustc_serialize;
extern crate docopt;

mod commands;

use std::error::Error;
use docopt::Docopt;

static USAGE: &'static str = "
Athena's project build system.

Usage:
    zeus <command> [<args>...]
    zeus

Some common zeus commands are:
    version     Display version info and exit
    list        Display a list of commands
    new         Create a new athena project
    setup       Sets up all athena tools for this project

See 'zeus help <command>' for more information on a specific command.
";

#[derive(RustcDecodable, Debug)]
struct Flags {
    arg_command: String,
    arg_args: Vec<String>
}

fn main() {
    // Parse in the command line flags
    let flags: Flags = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    // Run the actual command
    let result = match &flags.arg_command[..] {
        "list" => commands::list::execute(),
        "new" => commands::new::execute(),
        "setup" => commands::setup::execute(),
        "" => display_usage(),
        _ => display_not_found()
    };

    // Set the exit code depending on the result
    match result {
        Ok(_) => std::process::exit(0),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}


// ### Misc Command Handlers ###

fn display_usage() -> Result<(), Box<Error>> {
    println!("{}", USAGE);
    return Ok(());
}

fn display_not_found() -> Result<(), Box<Error>> {
    unimplemented!();
}
