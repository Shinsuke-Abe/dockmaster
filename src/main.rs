extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name>
    dockmaster (-h | --help)
    dockmaster --version

Options:
    -h --help   Show this screen.
    --version   Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_project_name: String,
    cmd_create: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    if args.cmd_create {
        println!("  createing {}", args.arg_project_name);
    }
}
