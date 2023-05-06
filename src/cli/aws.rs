use super::environments::NotifyEnvironment;
use std::process::Command;

pub fn exec(environment: NotifyEnvironment, exec_command: Vec<String>) {
    let env = match environment {
        NotifyEnvironment::DEV => "notify-tools",
        NotifyEnvironment::PREVIEW => "notify-preview",
        NotifyEnvironment::STAGING => "notify-staging",
        NotifyEnvironment::PRODUCTION => "notify-prod",
    };

    let mut full_command: Vec<String> = vec!["aws", env, "--"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    full_command.extend(exec_command);

    match Command::new("gds").args(full_command).status() {
        Err(_) => {}
        Ok(_) => {
            println!("Done")
        }
    }
}

pub fn console(environment: NotifyEnvironment) {
    let env = match environment {
        NotifyEnvironment::DEV => "notify-tools",
        NotifyEnvironment::PREVIEW => "notify-preview",
        NotifyEnvironment::STAGING => "notify-staging",
        NotifyEnvironment::PRODUCTION => "notify-prod",
    };

    match Command::new("gds").args(["aws", env, "-l"]).status() {
        Err(_) => {}
        Ok(_) => {
            println!("Done")
        }
    }
}
