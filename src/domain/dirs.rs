use std::env;
use std::path::PathBuf;

const APP_BASE_NAME: &'static str="dockermaster";

pub fn application_base() -> PathBuf {
    // TODO not default base directory -> environment value
    // TODO result http://osamu0329.hatenablog.jp/entry/2015/05/10/021234
    env::home_dir().unwrap().join(APP_BASE_NAME)
}