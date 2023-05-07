use super::args::DbArgs;
use super::enums::{InfrastructureTarget, NotifyEnvironment};
use std::process::{exit, Command};

fn db_connect_paas(environment: NotifyEnvironment, admin: bool, command: Vec<String>) {
    match Command::new("cf")
        .args(["target", "-s", environment.to_string().as_str()])
        .status()
    {
        Err(_) => {
            panic!("Could not target environment: {}", environment.to_string())
        }
        Ok(status) => {
            if !status.success() {
                println!("Could not target environment: {}", environment.to_string());
                exit(1);
            };
        }
    }

    let base_command = match admin {
        false => vec!["conduit", "notify-db", "-c", "{\"read_only\": true}", "--"],
        true => vec!["conduit", "notify-db", "--"],
    };
    let mut full_command: Vec<String> = base_command.into_iter().map(|s| s.to_string()).collect();
    full_command.extend(command);

    match Command::new("cf").args(full_command).status() {
        Err(_) => {}
        Ok(_) => {}
    }
}

pub fn connect(args: DbArgs) {
    match args.infra {
        InfrastructureTarget::PAAS => db_connect_paas(args.environment, args.admin, args.command),
        InfrastructureTarget::AWS => {}
    };
}
