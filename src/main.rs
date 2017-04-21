extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::ffi::OsStr;

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

// TODO resource template -> https://github.com/Keats/tera
// TODO implement dockmaster trait
fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    std::process::exit(
        if args.cmd_create {
            create_project_base(args)
        } else if args.cmd_ls {
            list_all_projects()
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

/// create <project> sub command
fn create_project_base(args: Args) -> i32 {
    println!("  createing {}", args.arg_project_name);

    let mut base_dir = application_base_directory();
    base_dir.push(args.arg_project_name);

    if base_dir.exists() {
        println!("  project directory is already exists.");
        9
    } else {
        let _ = fs::create_dir_all(&mut base_dir);
        for sub_dir in &["apps", "env", "data", "bin"] {
            let _ = fs::create_dir_all(&mut base_dir.join(sub_dir));
        }
        0
    }
}

/// ls sub command
fn list_all_projects() -> i32 {
    println!("  listing projects");

    // TODO filter chain...
    // fs::read_dir(application_base_directory()).unwrap().filter(|p| p.unwrap().file_type().unwrap().is_dir())
    //                                                                ^^^^ cannot move out of borrowed content
    // caused by unwrap? http://qiita.com/tatsuya6502/items/10b4227beadf44f302fd
    for path in fs::read_dir(application_base_directory()).unwrap() {
        let unwraped_path = path.unwrap();

        if unwraped_path.file_type().unwrap().is_dir() {
            println!("  {}", unwraped_path.file_name().into_string().unwrap());
        }
    }

    0
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