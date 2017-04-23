use std::env;
use std::fs;
use std::path::PathBuf;

// TODO const?
fn application_base_directory() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join("dockermaster")
}

const SUB_DIRECTORIES:[&'static str; 4] = ["apps", "env", "data", "bin"];

pub trait DockmasterCommand {
    fn arg_project_name(&self) -> String;

    // TODO change Result<()> to return value for subcommand method
    // because role for defining return code is main function(application layer)

    /// create <project> sub command
    fn create_project_base(&self) -> i32 {
        println!("  createing {}", self.arg_project_name());

        let mut base_dir = application_base_directory();
        base_dir.push(self.arg_project_name()); // TODO join?

        if base_dir.exists() {
            println!("  project directory is already exists.");
            9
        } else {
            let _ = fs::create_dir_all(&mut base_dir);
            for sub_dir in &SUB_DIRECTORIES {
                let _ = fs::create_dir_all(&mut base_dir.join(sub_dir));
            }
            0
        }
    }

    /// ls sub command
    fn list_all_projects(&self) -> i32 {
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
}