use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::ffi::OsStr;

// TODO const?
fn application_base_directory() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join("dockermaster")
}

const SUB_DIRECTORIES:[&'static str; 4] = ["apps", "env", "data", "bin"];

/// project operation sub command base
macro_rules! project_operation {
    ($sel:ident; $operation:block) => (
        if $sel.project_dir().exists() {
            $operation;
            Ok(())
        } else {
            Err(String::from(format!("  project[{}] is not exists.", $sel.arg_project_name())))
        }
    )
}

pub trait DockmasterCommand {
    fn arg_project_name(&self) -> String;
    fn project_dir(&self) -> PathBuf {
        application_base_directory().join(self.arg_project_name())
    }

    // TODO change Result<()> to return value for subcommand method
    // because role for defining return code is main function(application layer)

    /// create <project> sub command
    fn create_project_base(&self) -> Result<(), String> {
        println!("  createing {}", self.arg_project_name());

        if self.project_dir().exists() {
            Err(String::from("  project directory is already exists."))
        } else {
            let _ = fs::create_dir_all(&mut self.project_dir());
            for sub_dir in &SUB_DIRECTORIES {
                let _ = fs::create_dir_all(&mut self.project_dir().join(sub_dir));
            }
            Ok(())
        }
    }

    /// ls sub command
    fn list_all_projects(&self) -> Result<(), String> {
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

        Ok(())
    }

    /// standby <project-name> sub command
    fn standby_project(&self) -> Result<(), String> {
        project_operation!(self; {
            self.execute_docker_compose(&["up", "-d"]);
            println!("export environment variables: source {}/env/{}.env",
                    self.project_dir().display(),
                    "default");
        })
    }

    /// terminate <project-name> sub command
    fn terminate_project(&self) -> Result<(), String> {
        project_operation!(self; {
            self.execute_docker_compose(&["stop"]);
        })
    }

    fn execute_docker_compose<I, S>(&self, commands: I)
        where I: IntoIterator<Item = S>,
            S: AsRef<OsStr>
    {
        let output = Command::new("docker-compose")
            .env("COMPOSE_FILE",
                &format!("{}/apps/docker-compose-{}.yml",
                        self.project_dir().display(),
                        "default"))
            .env("COMPOSE_PROJECT_NAME", self.arg_project_name())
            .args(commands)
            .output()
            .expect("failed to execute docker-compose");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}