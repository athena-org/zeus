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

mod utils;

use docopt::Docopt;
use utils::CommandResult;

static USAGE: &'static str = "
Usage:
    zeus [<command> [<args>]]

Some common zeus commands are:
    help        Display this message
    Version     Display version info and exit
";

#[derive(RustcDecodable, Debug)]
struct Flags {
    arg_command: String,
    arg_args: Vec<String>
}

fn main() {
    let flags: Flags = Docopt::new(USAGE)
                              .and_then(|d| d.decode())
                              .unwrap_or_else(|e| e.exit());

    // Run the actual command
    let result = match &flags.arg_command[..] {
        "" | "help" => execute_help(),
        _ => execute_notfound()
    };

    // Set the exit code depending on the result
    match result {
        CommandResult::Ok => std::process::exit(0),
        CommandResult::Err => std::process::exit(1)
    }
}

fn execute_help() -> CommandResult {
    println!("Help!");
    return CommandResult::Ok;
}

fn execute_notfound() -> CommandResult  {
    println!("Not found!");
    return CommandResult::Err;
}
