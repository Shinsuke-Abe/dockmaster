extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::env;
use std::fs;
use std::path::PathBuf;

const USAGE: &'static str = "
Dockmaster.

Usage:
    dockmaster create <project-name>
    dockmaster ls
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
    cmd_ls: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    
    if args.cmd_create {
        std::process::exit(create_project_base(args));
    } else if args.cmd_ls {
        println!("  listing projects");
        
        for path in fs::read_dir(application_base_directory()).unwrap() {
            let unwraped_path = path.unwrap();
            
            if unwraped_path.file_type().unwrap().is_dir() {
                println!("  {}", unwraped_path.file_name().into_string().unwrap());
            }
        }
    }
}

fn application_base_directory() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join("dockermaster")
}

fn create_project_base(args: Args) -> i32 {
    println!("  createing {}", args.arg_project_name);

    let mut base_dir = application_base_directory();
    base_dir.push(args.arg_project_name);
    println!("  project directory is {}", base_dir.display());

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