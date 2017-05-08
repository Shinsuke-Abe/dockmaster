extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate yamlette;

use docopt::Docopt;
use domain::DockmasterCommand;

mod domain;

// TODO execute product under developing
//   default => gradle + Spring Boot
//   set environment variable before execute application
// TODO connect product directory for project
// TODO execute product(customizing process)
// TODO exec command
//   set environment variable before execute command
//   injections, specify project and environment.
// TODO customize environment standby process
//   {name}.yml at project base directory
//     1) override process flag
//     2) process list
//     3) replace environment variable
// TODO customize executing application
// TODO named environment inherit other environment on customized process
// TODO remove project
// TODO project settings
// TODO check standby environment
//   standing environment
//     1) standby environment information file for .standing directory on application base
//     2) when exec standby command, create environment information file
//     3) when exec terminate command, delete environment information file
// TODO ps command
//   listing standing environment
// TODO listing images and using port for all project
//   from docker-compose.yml
const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name>
    dockmaster ls
    dockmaster standby <project-name> [--env=<env-name>] 
    dockmaster run product <project-name> [--tasks=<execute-task>]
    dockmaster terminate <project-name> [--env=<env-name>]
    dockmaster (-h | --help)
    dockmaster --version

Options:
    -h --help               Show this screen.
    --version               Show version.
    --env=<env-name>        Environment name for stand-by project [default: default].
    --tasks=<execute-task>  Task name for gradle on product under developing [default: run].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_project_name: String,
    flag_env: String,
    flag_tasks: String, 
    cmd_create: bool,
    cmd_ls: bool,
    cmd_standby: bool,
    cmd_run: bool,
    cmd_product: bool,
    cmd_terminate: bool,
}

impl DockmasterCommand for Args {
    fn project_name(&self) -> String {
        self.arg_project_name.clone()
    }

    fn env_name(&self) -> String {
        self.flag_env.clone()
    }

    fn task_name(&self) -> String {
        self.flag_tasks.clone()
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
    } else if args.cmd_run & args.cmd_product {
        result_handling!(args.run_product())
    } else if args.cmd_terminate {
        result_handling!(args.terminate_project())
    } else {
        std::process::exit(ERROR)
    }
}