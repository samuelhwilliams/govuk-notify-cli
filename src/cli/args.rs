use super::environments::NotifyEnvironment;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NotifyCliArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Interact with Notify's AWS infrastructure
    Aws(AwsCommand),

    /// Get a connection to one of Notify's databases
    Db(EnvironmentCommand),

    /// SSH to a Notify app instance
    Ssh(EnvironmentCommand),
}

#[derive(Debug, Args)]
pub struct AwsCommand {
    #[clap(subcommand)]
    pub subcommand: AwsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AwsSubcommand {
    // Execute a command in a given AWS account
    #[clap(visible_alias("e"))]
    Exec(EnvironmentCommand),

    // Launch the AWS Console in a browser window
    #[clap(visible_alias("c"))]
    Console(EnvironmentArg),
}

#[derive(Debug, Args)]
pub struct EnvironmentCommand {
    pub environment: NotifyEnvironment,

    #[clap(trailing_var_arg = true, allow_hyphen_values = true, num_args=1..)]
    pub command: Vec<String>,
}

#[derive(Debug, Args)]
pub struct EnvironmentArg {
    pub environment: NotifyEnvironment,
}
