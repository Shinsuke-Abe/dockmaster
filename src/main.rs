extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name> [--base=<base-dir>]
    dockmaster (-h | --help)
    dockmaster --version

Options:
    --base=<base-dir>   project base directory(default is current directory).
    -h --help           Show this screen.
    --version           Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_base: String,
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
        if !args.flag_base.is_empty() {
            println!("  base directory is {}", args.flag_base);
        } else {
            println!("  base directory is {}", env::current_dir().unwrap().display());
        }
    }
}
