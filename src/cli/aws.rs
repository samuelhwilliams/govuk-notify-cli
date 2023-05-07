use super::args::{AwsConsoleArgs, AwsExecArgs};
use super::helpers::{confirm_cyber_approval, get_account_name_from_environment};
use std::process::Command;

pub fn exec(args: AwsExecArgs) {
    let env = get_account_name_from_environment(args.environment, args.admin);

    let mut full_command: Vec<String> = vec!["aws", env, "--"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    full_command.extend(args.command);

    match Command::new("gds").args(full_command).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}

pub fn console(args: AwsConsoleArgs) {
    let env = get_account_name_from_environment(args.environment, args.admin);

    confirm_cyber_approval(args.environment, args.admin);

    match Command::new("gds").args(["aws", env, "-l"]).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}
