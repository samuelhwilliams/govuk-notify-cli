use std::path::PathBuf;

use super::enums::NotifyEnvironment;
use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(name = "notify", author, version, about, arg_required_else_help = true)]
pub struct NotifyCliArgs {
    #[arg(long = "generate", value_enum)]
    pub generator: Option<Shell>,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Interact with Notify's AWS infrastructure
    Aws(AwsCommand),

    /// Get a connection to one of Notify's databases
    Db(DbArgs),

    /// SSH to a Notify app instance
    Ssh(SshArgs),
}

#[derive(Debug, Args)]
pub struct AwsCommand {
    #[clap(subcommand)]
    pub subcommand: AwsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AwsSubcommand {
    /// Execute a command in a given AWS account
    Exec(AwsExecArgs),

    /// Launch the AWS Console in a browser window
    Console(AwsConsoleArgs),
}

#[derive(Debug, Args)]
pub struct AwsExecArgs {
    /// The environment to target
    pub environment: NotifyEnvironment,

    /// Use the admin role with write access
    #[clap(long = "admin")]
    pub admin: bool,

    /// The command to run
    #[clap(last = true, allow_hyphen_values = true, num_args=1.., required = true)]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct AwsConsoleArgs {
    /// The environment to target
    pub environment: NotifyEnvironment,

    /// Use the admin role with write access
    #[clap(long = "admin")]
    pub admin: bool,
}

#[derive(Debug, Args)]
pub struct DbArgs {
    pub environment: NotifyEnvironment,

    // Target AWS infrastructure instead
    #[clap(long, default_value_t = false)]
    pub paas: bool,

    /// Connect with write access
    #[clap(long = "write", default_value_t=false)]
    pub write: bool,

    /// Connect with admin access
    #[clap(long = "admin", default_value_t=false)]
    pub admin: bool,

    #[clap(
        long = "aws-repo",
        required = false,
        env = "NOTIFY_AWS",
        required_if_eq("paas", "false")
    )]
    /// Path to your local checkout of the notifications-aws repo
    pub aws_repo: PathBuf,

    /// The command to run
    #[clap(last = true, allow_hyphen_values = true, num_args=1.., default_value = "psql")]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SshArgs {
    // Target AWS infrastructure instead
    #[clap(long, default_value_t = false)]
    pub aws: bool,

    /// The environment to target
    pub environment: NotifyEnvironment,

    /// The name of the service to SSH onto
    #[clap(default_value = "notify-api", default_value_if("aws", "true", "api"))]
    pub service_name: String,
}
