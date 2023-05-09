use super::args::DbArgs;
use super::enums::NotifyEnvironment;
use super::helpers::{
    cf_ensure_logged_in_and_target_space, confirm_cyber_approval, get_account_name_from_environment,
};
use std::path::PathBuf;
use std::process::Command;

const PAAS_DB_NAME: &str = "notify-db";
const AWS_DB_NAME: &str = "notifydb";

fn db_connect_paas(environment: NotifyEnvironment, allow_writes: bool, command: Vec<String>) {
    cf_ensure_logged_in_and_target_space(environment);

    let base_command = match allow_writes {
        true => vec!["conduit", PAAS_DB_NAME, "--"],
        false => vec!["conduit", PAAS_DB_NAME, "-c", "{\"read_only\": true}", "--"],
    };
    let mut full_command: Vec<String> = base_command.into_iter().map(|s| s.to_string()).collect();
    full_command.extend(command);

    match Command::new("cf").args(full_command).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}

fn db_connect_aws(environment: NotifyEnvironment, command: Vec<String>, aws_repo_path: PathBuf) {
    let aws_account_name = get_account_name_from_environment(&environment, true);

    let executable = aws_repo_path
        .join("scripts")
        .join("db-connect")
        .join("db-connect.sh")
        .to_string_lossy()
        .to_string();
    let mut args: Vec<String> = vec![
        "aws",
        aws_account_name,
        "--",
        executable.as_str(),
        AWS_DB_NAME,
        "--",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();
    args.extend(command);

    match Command::new("gds").args(args).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}

pub fn connect(args: DbArgs) {
    match args.aws {
        true => {
            // We don't support readonly access here yet, WIP.
            confirm_cyber_approval(args.environment, true);
            db_connect_aws(args.environment, args.command, args.aws_repo);
        }
        false => {
            confirm_cyber_approval(args.environment, args.allow_writes);
            db_connect_paas(args.environment, args.allow_writes, args.command);
        }
    };
}
