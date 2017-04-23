extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::ffi::OsStr;
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
// TODO implement dockmaster trait
// -> http://qiita.com/mandel59/items/e9a5438f4c1d70cffb7a
// -> http://rust-lang-ja.org/rust-by-example/trait.html
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
            project_operation(&args, &standby_project)
        } else if args.cmd_terminate {
            project_operation(&args, &terminate_project)
        } else {
            0
        });
}

fn application_base_directory() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join("dockermaster")
}

/// project operation sub command base
fn project_operation(args: &Args, operations: &Fn(&Args) -> ()) -> i32 {
    let project_dir = application_base_directory().join(&args.arg_project_name);
    if project_dir.exists() {
        operations(args);
        0
    } else {
        println!("  project[{}] is not exists.", args.arg_project_name);
        9
    }
}

/// standby <project-name> sub command
fn standby_project(args: &Args) {
    let project_dir = application_base_directory().join(&args.arg_project_name);
    execute_docker_compose(args, &["up", "-d"]);
    println!("export environment variables: source {}/env/{}.env",
             &project_dir.display(),
             "default");
}

/// terminate <project-name> sub command
fn terminate_project(args: &Args) {
    execute_docker_compose(args, &["stop"]);
}

fn execute_docker_compose<I, S>(args: &Args, commands: I) where I: IntoIterator<Item=S>, S: AsRef<OsStr> {
    let project_dir = application_base_directory().join(&args.arg_project_name);
    let output = Command::new("docker-compose")
        .env("COMPOSE_FILE",
             &format!("{}/apps/docker-compose-{}.yml",
                      &project_dir.display(),
                      "default"))
        .env("COMPOSE_PROJECT_NAME", &args.arg_project_name)
        .args(commands)
        .output()
        .expect("failed to execute docker-compose");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}