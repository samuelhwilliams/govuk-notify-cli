use ansi_term::Color;
use std::io;
use std::io::Write;
use std::process::{exit, Command, Stdio};

use super::enums::NotifyEnvironment;

const CF_API_DOMAIN: &str = "api.cloud.service.gov.uk";
const CF_ORG_NOTIFY: &str = "govuk-notify";

pub fn get_account_name_from_environment(
    environment: &NotifyEnvironment,
    admin: bool,
) -> &'static str {
    match admin {
        true => match environment {
            NotifyEnvironment::DEV => return "notify-tools-admin",
            NotifyEnvironment::PREVIEW => return "notify-preview-admin",
            NotifyEnvironment::STAGING => return "notify-staging-admin",
            NotifyEnvironment::PROD | NotifyEnvironment::PRODUCTION => return "notify-prod-admin",
            NotifyEnvironment::DEPLOY => return "notify-deploy-admin",
        },
        false => match environment {
            NotifyEnvironment::DEV => return "notify-tools",
            NotifyEnvironment::PREVIEW => return "notify-preview",
            NotifyEnvironment::STAGING => return "notify-staging",
            NotifyEnvironment::PROD | NotifyEnvironment::PRODUCTION => return "notify-prod",
            NotifyEnvironment::DEPLOY => return "notify-deploy",
        },
    };
}

pub fn cf_ensure_logged_in_and_target_space(environment: NotifyEnvironment) {
    let exitcode = Command::new("cf")
        .arg("oauth-token")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();

    match exitcode.success() {
        true => {
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
        }
        false => {
            let exitcode = Command::new("cf")
                .args([
                    "login",
                    "-a",
                    CF_API_DOMAIN,
                    "--sso",
                    "-o",
                    CF_ORG_NOTIFY,
                    "-s",
                    environment.to_string().as_str(),
                ])
                .status()
                .unwrap();

            if !exitcode.success() {
                println!(
                    "\n{}: Could not log into CF CLI. Please try manually: `cf login -a {} --sso`",
                    Color::Red.paint("FAILED"),
                    CF_API_DOMAIN,
                );
                exit(1);
            }
        }
    }
}

pub fn confirm_cyber_approval(environment: NotifyEnvironment, admin: bool) {
    match (environment, admin) {
        (NotifyEnvironment::PRODUCTION, true) => {
            let mut answer = String::new();
            print!(
                "\n{}: This action will trigger a cyber alert. Have you submitted a 2-eye approval? [y/N] ",
                Color::Red.paint("WARNING")
            );
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut answer)
                .expect("Error reading input.");

            match answer.as_str().trim() {
                "y" | "yes" => {}
                _ => {
                    println!(
                        "-> Submit a cyber 2-eye approval in the #govuk-cyber-approvals channel."
                    );
                    exit(1);
                }
            }
        }
        _ => {}
    };
}
