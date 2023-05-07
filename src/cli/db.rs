use super::args::DbArgs;
use super::enums::{InfrastructureTarget, NotifyEnvironment};
use std::process::{exit, Command};

fn db_connect_paas(environment: NotifyEnvironment, allow_writes: bool, command: Vec<String>) {
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

    let base_command = match allow_writes {
        true => vec!["conduit", "notify-db", "--"],
        false => vec!["conduit", "notify-db", "-c", "{\"read_only\": true}", "--"],
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
        InfrastructureTarget::PAAS => {
            db_connect_paas(args.environment, args.allow_writes, args.command)
        }
        InfrastructureTarget::AWS => {}
    };
}
