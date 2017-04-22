use std::env;
use std::fs;
use std::path::PathBuf;

fn application_base_directory() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join("dockermaster")
}

pub trait DockmasterCommand {
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