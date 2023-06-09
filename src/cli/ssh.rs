use std::process::Command;

use crate::cli::helpers::cf_ensure_logged_in_and_target_space;

use super::{
    args::SshArgs,
    enums::NotifyEnvironment,
    helpers::{confirm_cyber_approval, get_account_name_from_environment},
};

const CLUSTER_NAME: &str = "notify";

fn aws_ssh_via_ecs_exec(environment: NotifyEnvironment, service_name: String) {
    let aws_account_name = get_account_name_from_environment(&environment, true);

    println!("Using AWS role: {}", aws_account_name.to_lowercase());

    let running_task_arn = String::from_utf8(
        Command::new("gds")
            .args([
                "aws",
                aws_account_name,
                "--",
                "aws",
                "ecs",
                "list-tasks",
                "--cluster",
                CLUSTER_NAME,
                "--service-name",
                service_name.as_str(),
                "--desired-status",
                "RUNNING",
                "--query",
                "taskArns[0]",
                "--output",
                "text",
            ])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let running_task_id = running_task_arn.split("/").last().unwrap().trim();

    println!(
        "Latest running task ID for service {}: {}. Connecting ...",
        service_name, running_task_id
    );

    Command::new("gds")
        .args([
            "aws",
            aws_account_name,
            "--",
            "aws",
            "ecs",
            "execute-command",
            "--cluster",
            CLUSTER_NAME,
            "--task",
            running_task_id,
            "--container",
            service_name.as_str(),
            "--interactive",
            "--command",
            "/bin/bash",
        ])
        .status()
        .unwrap();
}

fn paas_ssh_via_cf(environment: NotifyEnvironment, service_name: String) {
    cf_ensure_logged_in_and_target_space(environment);

    println!("Connecting to {} ...", service_name);

    let full_command = vec!["ssh".to_string(), service_name];
    match Command::new("cf").args(full_command).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}

pub fn connect(args: SshArgs) {
    confirm_cyber_approval(args.environment, true);

    match args.aws {
        true => aws_ssh_via_ecs_exec(args.environment, args.service_name),
        false => paas_ssh_via_cf(args.environment, args.service_name),
    }
}
