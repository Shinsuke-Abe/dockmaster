extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use domain::DockmasterCommand;

mod domain;

const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name>
    dockmaster ls
    dockmaster standby <project-name> [--env=<env-name>] 
    dockmaster terminate <project-name>
    dockmaster (-h | --help)
    dockmaster --version

Options:
    -h --help           Show this screen.
    --version           Show version.
    --env=<env-name>    Environment name for stand-by project [default: default].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_project_name: String,
    cmd_create: bool,
    cmd_ls: bool,
    cmd_standby: bool,
    cmd_terminate: bool,
}

impl DockmasterCommand for Args {
    fn arg_project_name(&self) -> String {
        self.arg_project_name.clone()
    }
}

// TODO resource template -> https://github.com/Keats/tera
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    std::process::exit(
        if args.cmd_create {
            args.create_project_base()
        } else if args.cmd_ls {
            args.list_all_projects()
        } else if args.cmd_standby {
            args.standby_project()
        } else if args.cmd_terminate {
            args.terminate_project()
        } else {
            0
        }
    );
}