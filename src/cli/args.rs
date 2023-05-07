use std::path::PathBuf;

use super::enums::{InfrastructureTarget, NotifyEnvironment};
use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
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
    #[clap(visible_alias("e"))]
    Exec(AwsExecArgs),

    /// Launch the AWS Console in a browser window
    #[clap(visible_alias("c"))]
    Console(AwsConsoleArgs),
}

#[derive(Debug, Args)]
pub struct EnvironmentCommand {
    /// The environment to target
    pub environment: NotifyEnvironment,

    /// The command to run
    #[clap(trailing_var_arg = true, allow_hyphen_values = true, num_args=1..)]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct AwsExecArgs {
    /// The environment to target
    pub environment: NotifyEnvironment,

    /// Use the admin role with write access
    #[clap(long = "admin")]
    pub admin: bool,

    /// The command to run
    #[clap(trailing_var_arg = true, allow_hyphen_values = true, num_args=1.., required = true)]
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

    #[clap(long = "admin")]
    pub admin: bool,

    /// The command to run
    #[clap(trailing_var_arg = true, allow_hyphen_values = true, num_args=1.., default_value = "psql")]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct SshArgs {
    #[clap(value_enum, long="infra", default_value_t = InfrastructureTarget::PAAS)]
    pub infra: InfrastructureTarget,

    /// The environment to target
    pub environment: NotifyEnvironment,

    #[clap(
        long = "aws-repo",
        required = false,
        env = "NOTIFY_AWS",
        required_if_eq("infra", "AWS")
    )]
    /// Path to your local checkout of the notifications-aws repo
    pub aws_repo: PathBuf,

    /// The name of the service to SSH onto
    #[clap(default_value = "notify-api", default_value_if("infra", "aws", "api"))]
    pub service_name: String,
}
