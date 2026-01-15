use crate::parser::ast::Command;

mod cd;
mod exit;
mod jobs;

pub enum BuiltinResult {
    Handled,
    ExitShell,
    NotBuiltin,
}

pub fn handle_builtin(cmd: &Command) -> BuiltinResult {
    match cmd.program.as_str() {
        "cd" => {
            cd::run(cmd);
            BuiltinResult::Handled
        }
        "exit" => BuiltinResult::ExitShell,
        _ => BuiltinResult::NotBuiltin,
    }
}
