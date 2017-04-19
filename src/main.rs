extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;
use std::fs;

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
        std::process::exit(create_project_base(args));
    }
}

fn create_project_base(args: Args) -> i32 {
    println!("  createing {}", args.arg_project_name);

    let mut base_dir = env::home_dir().unwrap();

    // TODO not default base directory -> environment value
    base_dir.push("dockermaster");
    base_dir.push(args.arg_project_name);
    println!("  project directory is {}", base_dir.display());

    if base_dir.exists() {
        println!("  project directory is already exists.");
        9
    } else {
        let _ = fs::create_dir_all(&mut base_dir);
        let _ = fs::create_dir_all(&mut base_dir.join("apps"));
        let _ = fs::create_dir_all(&mut base_dir.join("env"));
        let _ = fs::create_dir_all(&mut base_dir.join("data"));
        let _ = fs::create_dir_all(&mut base_dir.join("bin"));
        0
    }
}