use std::env;
use std::path::PathBuf;

const APP_BASE_NAME: &'static str="dockermaster";

pub fn application_base() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join(APP_BASE_NAME)
}

pub struct Project {
    name: String,
}

impl Project {
    pub fn named(name: String) -> Project {
        Project{name: name}
    }

    pub fn base(&self) -> PathBuf {
        application_base().join(&self.name)
    }

    pub fn apps(&self) -> PathBuf {
        self.base().join("apps")
    }

    pub fn env(&self) -> PathBuf {
        self.base().join("env")
    }
}
