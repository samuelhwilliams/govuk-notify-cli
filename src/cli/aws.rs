use super::args::{AwsConsoleArgs, AwsExecArgs};
use super::enums::NotifyEnvironment;
use std::process::Command;

fn get_account_name_from_environment(environment: NotifyEnvironment, admin: bool) -> &'static str {
    let base_role_name = match environment {
        NotifyEnvironment::DEV => {
            if admin {
                "notify-tools-admin"
            } else {
                "notify-tools"
            }
        }
        NotifyEnvironment::PREVIEW => {
            if admin {
                "notify-preview-admin"
            } else {
                "notify-preview"
            }
        }
        NotifyEnvironment::STAGING => {
            if admin {
                "notify-staging-admin"
            } else {
                "notify-staging"
            }
        }
        NotifyEnvironment::PRODUCTION => {
            if admin {
                "notify-prod-admin"
            } else {
                "notify-prod"
            }
        }
    };

    base_role_name
}

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

    match Command::new("gds").args(["aws", env, "-l"]).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}
