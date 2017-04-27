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
            Err(String::from(format!("  project[{}] is not exists.", $sel.project_name())))
        }
    )
}

macro_rules! handling_command_error {
    ($command_execution:expr) => {
        if let Err(e) = $command_execution {
            return Err(e);
        }
    }
}

pub trait DockmasterCommand {
    fn project_name(&self) -> String;

    fn env_name(&self) -> String;

    fn project_dir(&self) -> PathBuf {
        application_base_directory().join(self.project_name())
    }

    fn docker_compose_file_with_env(&self) -> PathBuf {
        self.project_dir().join("apps").join(format!("docker-compose-{}.yml", self.env_name()))
    }

    fn environment_file_with_env(&self) -> PathBuf {
        self.project_dir().join("env").join(format!("{}.env", self.env_name()))
    }

    /// create <project> sub command
    fn create_project_base(&self) -> Result<(), String> {
        println!("  createing {}", self.project_name());

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
        // TODO if {named-environment}.yml (inheritance)
        project_operation!(self; {
            handling_command_error!(self.execute_docker_compose(&["up", "-d"]));
            if self.environment_file_with_env().exists() {
                println!(
                    "export environment variables: source {}",
                    self.environment_file_with_env().display());
            } else {
                return Err(String::from("environment variable file is not found"));
            }
        })
    }

    /// terminate <project-name> sub command
    fn terminate_project(&self) -> Result<(), String> {
        // TODO if {named-environment}.yml (inheritance)
        project_operation!(self; {
            handling_command_error!(self.execute_docker_compose(&["stop"]))
        })
    }

    fn execute_docker_compose<I, S>(&self, commands: I) -> Result<(), String>
        where I: IntoIterator<Item = S>,
            S: AsRef<OsStr>
    {
        let output = Command::new("docker-compose")
            .env("COMPOSE_FILE", self.docker_compose_file_with_env().into_os_string())
            .env("COMPOSE_PROJECT_NAME", self.project_name())
            .args(commands)
            .output()
            .expect("failed to execute docker-compose");

        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from("failed to execute docker-compose"))
        }  
    }
}