extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate yamlette;

use docopt::Docopt;
use domain::DockmasterCommand;

mod domain;

// TODO named environment inherit other environment on default process
//   https://github.com/dnsl48/yamlette
//   http://qiita.com/tatsuya6502/items/bed3702517b36afbdbca
//   {name}.yml at project base directory
//     1) inherit base environment name
//     1)-1 {name}.yml
//     1)-2 parent: {parent_name}
//     1)-3 process:
//     1)-4   default: true|false
//     1)-4   compose: parent|this
//     1)-5   env: parent|this
//     2) search {name}.yml
//     3) if not named yml, execute default process
// TODO customize environment standby process
//   {name}.yml at project base directory
//     1) override process flag
//     2) process list
//     3) replace environment variable
// TODO named environment inherit other environment on customized process
// TODO exec command
//   set environment variable before execute command
//   injections, specify project and environment.
// TODO remove project
// TODO execute application
// TODO project settings
// TODO check standby environment
//   standing environment
//     1) standby environment information file for .standing directory on application base
//     2) when exec standby command, create environment information file
//     3) when exec terminate command, delete environment information file
// TODO ps command
//   listing standing environment
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
    flag_env: String,
    cmd_create: bool,
    cmd_ls: bool,
    cmd_standby: bool,
    cmd_terminate: bool,
}

impl DockmasterCommand for Args {
    fn project_name(&self) -> String {
        self.arg_project_name.clone()
    }

    fn env_name(&self) -> String {
        self.flag_env.clone()
    }
}

const SUCCESS: i32 = 0;
const ERROR: i32 = 9;

macro_rules! result_handling {
    ($op: expr) => (
        match $op {
            Ok(()) => std::process::exit(SUCCESS),
            Err(e) => {
                println!("{}", e);
                std::process::exit(ERROR)
            }
        }
    )
}

// TODO resource template -> https://github.com/Keats/tera
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_create {
        result_handling!(args.create_project_base())
    } else if args.cmd_ls {
        result_handling!(args.list_all_projects())
    } else if args.cmd_standby {
        result_handling!(args.standby_project())
    } else if args.cmd_terminate {
        result_handling!(args.terminate_project())
    } else {
        std::process::exit(ERROR)
    }
}