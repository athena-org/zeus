use docopt::Docopt;
use utils::CommandResult;

static USAGE: &'static str = "
Usage:
    zeus [help [<command>]]
";

#[derive(RustcDecodable, Debug)]
struct Flags {
    arg_command: String
}

pub fn execute() -> CommandResult {
    // Parse in the command line flags
    let flags: Flags = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Help!");
    return CommandResult::Ok;
}
