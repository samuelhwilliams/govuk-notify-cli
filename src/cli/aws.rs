use super::args::{AwsConsoleArgs, AwsExecArgs};
use super::environments::NotifyEnvironment;
use std::process::Command;

fn get_account_name_from_environment(environment: NotifyEnvironment) -> &'static str {
    match environment {
        NotifyEnvironment::DEV => return "notify-tools",
        NotifyEnvironment::PREVIEW => return "notify-preview",
        NotifyEnvironment::STAGING => return "notify-staging",
        NotifyEnvironment::PRODUCTION => return "notify-prod",
    }
}

pub fn exec(args: AwsExecArgs) {
    let env = get_account_name_from_environment(args.environment);

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
    let env = get_account_name_from_environment(args.environment);

    match Command::new("gds").args(["aws", env, "-l"]).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}
