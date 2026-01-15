use crate::parser::ast::Command;
use std::env;

pub fn run(cmd: &Command) {
    let target = cmd
        .args
        .get(0)
        .map(String::as_str)
        .unwrap_or("/");

    if let Err(e) = env::set_current_dir(target) {
        eprintln!("cd: {}", e);
    }
}
