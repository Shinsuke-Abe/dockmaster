extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;

const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name>
    dockmaster (-h | --help)
    dockmaster --version

Options:
    -h --help           Show this screen.
    --version           Show version.
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

        let mut base_dir = env::home_dir().unwrap();

        base_dir.push("dockermaster");
        // TODO not default base directory -> environment value
        println!("  base directory is {}", base_dir.display());

        base_dir.push(args.arg_project_name);
        println!("  project directory is {}", base_dir.display());
        // TODO create project sub tree
        //     project-base > apps > docker-compose_default.yml
        //                  > env > default.env
        //                  > data
        //                  > bin
    }
}
