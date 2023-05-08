use std::path::PathBuf;

use super::enums::{InfrastructureTarget, NotifyEnvironment};
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

    /// The environment to target
    #[clap(value_enum, long="infra", default_value_t = InfrastructureTarget::PAAS)]
    pub infra: InfrastructureTarget,

    /// Connect with write access
    #[clap(long = "allow-writes")]
    pub allow_writes: bool,

    #[clap(
        long = "aws-repo",
        required = false,
        env = "NOTIFY_AWS",
        required_if_eq("infra", "AWS")
    )]
    /// Path to your local checkout of the notifications-aws repo
    pub aws_repo: PathBuf,

    /// The command to run
    #[clap(last = true, allow_hyphen_values = true, num_args=1.., default_value = "psql")]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SshArgs {
    #[clap(value_enum, long="infra", default_value_t = InfrastructureTarget::PAAS)]
    pub infra: InfrastructureTarget,

    /// The environment to target
    pub environment: NotifyEnvironment,

    /// The name of the service to SSH onto
    #[clap(default_value = "notify-api", default_value_if("infra", "aws", "api"))]
    pub service_name: String,
}
