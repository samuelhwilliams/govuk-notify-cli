use std::process::Command;

use super::{
    args::SshArgs,
    enums::{InfrastructureTarget, NotifyEnvironment},
    helpers::get_account_name_from_environment,
};

const CLUSTER_NAME: &str = "notify";

fn aws_ssh_via_ecs_exec(environment: NotifyEnvironment, service_name: String) {
    let aws_account_name = get_account_name_from_environment(environment, true);

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
        "Latest running task ID for service {}: {}. Connecting...",
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

pub fn connect(args: SshArgs) {
    match args.infra {
        InfrastructureTarget::PAAS => {}
        InfrastructureTarget::AWS => aws_ssh_via_ecs_exec(args.environment, "api".to_string()),
    }
}
