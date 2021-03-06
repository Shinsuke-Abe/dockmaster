use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

pub mod dirs;

struct EnvironmentSettings {
    parent: String,
    process_default: bool,
    process_compose: String,
    process_env: String,
}

fn load_environment_settings(settings_path: PathBuf) -> EnvironmentSettings {
    let file = File::open(settings_path).unwrap();
    let buf_reader = BufReader::new(file);
    yamlette!(read ; buf_reader ; [[
        {
            "parent" => (parent: String),
            "process" => {
                "default" => (process_default: bool),
                "compose" => (process_compose: String),
                "env" => (process_env: String)
            }
        }
    ]]);

    EnvironmentSettings{
        parent: parent.unwrap_or(String::from("default")),
        process_default: process_default.unwrap_or(true),
        process_compose: process_compose.unwrap_or(String::from("this")),
        process_env: process_env.unwrap_or(String::from("this"))}
}

fn load_product_settings(settings_path: PathBuf) -> Option<PathBuf> {
    let file = File::open(settings_path).unwrap();
    let buf_reader = BufReader::new(file);
    yamlette!(read ; buf_reader ; [[
        {
            "execution_base" => (execution_base: String)
        }
    ]]);

    match execution_base {
        Some(execution_base) => {
            let execution_base_path = PathBuf::from(execution_base);

            if execution_base_path.exists() {
                Some(execution_base_path)
            } else {
                None
            }
        },
        None => None
    }
}

#[derive(Debug)]
pub enum ProcessOnDefault {
    Compose,
    Env,
}

/// project operation sub command base
macro_rules! project_operation {
    ($sel:ident; $operation:block) => (
        if dirs::Project::named($sel.project_name()).base().exists() {
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

macro_rules! decide_env_name {
    ($sel:ident; $process_setting:expr; $parent_name:expr) => {
        if $process_setting == "parent" {
            $parent_name
        } else {
            $sel.env_name()
        }
    }
}

// TODO create command builder and template method
pub trait DockmasterCommand {
    fn project_name(&self) -> String;

    fn env_name(&self) -> String;

    fn task_name(&self) -> String;

    fn actual_env_name(&self, process: ProcessOnDefault) -> String {
        let settings_path = dirs::Project::named(self.project_name()).base().join(format!("{}.yml", self.env_name()));
        if settings_path.exists() {
            let settings = load_environment_settings(settings_path);
            
            match process {
                ProcessOnDefault::Compose =>
                    decide_env_name!(self; settings.process_compose; settings.parent),
                ProcessOnDefault::Env => 
                    decide_env_name!(self; settings.process_env; settings.parent)
            }
        } else {
            self.env_name()
        }
    }

    fn docker_compose_file_with_env(&self) -> PathBuf {
        dirs::Project::named(self.project_name()).apps().join(format!("docker-compose-{}.yml", self.actual_env_name(ProcessOnDefault::Compose)))
    }

    fn environment_file_with_env(&self) -> PathBuf {
        dirs::Project::named(self.project_name()).env().join(format!("{}.env", self.actual_env_name(ProcessOnDefault::Env)))
    }

    /// create <project> sub command
    fn create_project_base(&self) -> Result<(), String> {
        println!("  createing {}", self.project_name());

        if dirs::Project::named(self.project_name()).base().exists() {
            Err(String::from("  project directory is already exists."))
        } else {
            for sub_dir in &dirs::Project::named(self.project_name()).to_subdir_arr() {
                let _ = fs::create_dir_all(&sub_dir);
            }
            Ok(())
        }
    }

    /// ls sub command
    fn list_all_projects(&self) -> Result<(), String> {
        println!("  listing projects");

        for path in fs::read_dir(dirs::application_base()).unwrap() {
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
            handling_command_error!(self.execute_docker_compose(&["up", "-d"]));
            let env_file = self.environment_file_with_env();
            if env_file.exists() {
                println!(
                    "export environment variables: source {}",
                    env_file.display());
            } else {
                return Err(String::from("environment variable file is not found"));
            }
        })
    }

    /// run product sub command
    fn run_product(&self) -> Result<(), String> {
        project_operation!(self; {
            let settings_path = dirs::Project::named(self.project_name()).base().join("product_settings.yml");

            if !settings_path.exists() {
                return Err(String::from("product setting is not found"));
            }

            match load_product_settings(settings_path) {
                Some(execution_base_path) => {
                    // load environment variable
                    let env_file_path = self.environment_file_with_env();
                    if env_file_path.exists() {
                        let mut env_file_lines = BufReader::new(File::open(env_file_path).unwrap()).lines()
                            .map(|line| line.unwrap())
                            .filter(|line| line.starts_with("export"))
                            .map(|line| line.replace("export", "").trim().to_string());

                        while let Some(line) = env_file_lines.next() {
                            env::set_var(String::from(line.split("=").nth(0).unwrap()), line.split("=").nth(1).unwrap().replace("\"", ""));
                        }
                    }

                    println!("if you want to stop application, type [end]\n");

                    if let Ok(mut child) = Command::new("./gradlew")
                        .current_dir(execution_base_path)
                        .arg(self.task_name())
                        .spawn() {
                        loop {
                            let mut buf = String::new();
                            match io::stdin().read_line(&mut buf) {
                                Ok(_) => {
                                    if "end\n" == buf {
                                        child.kill().expect("gradle command not running");
                                        break;
                                    }
                                },
                                Err(e) => println!("{}", e)
                            }
                        }
                        Ok(())
                    } else {
                        Err(String::from("failed to execute gradle"))
                    }
                },
                None => return Err(String::from("execution path is not set"))
            }
        })
    }

    /// terminate <project-name> sub command
    fn terminate_project(&self) -> Result<(), String> {
        project_operation!(self; {
            handling_command_error!(self.execute_docker_compose(&["stop"]))
        })
    }

    fn execute_docker_compose<I, S>(&self, commands: I) -> Result<(), String>
        where I: IntoIterator<Item = S>,
            S: AsRef<OsStr>
    {
        // TODO use status
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