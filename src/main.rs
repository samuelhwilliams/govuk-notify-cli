mod cli;

use clap::Parser;

fn main() {
    let args = cli::args::NotifyCliArgs::parse();

    match args.command {
        cli::args::Command::Aws(command) => match command.subcommand {
            cli::args::AwsSubcommand::Exec(args) => cli::aws::exec(args),
            cli::args::AwsSubcommand::Console(args) => cli::aws::console(args),
        },
        cli::args::Command::Db(args) => cli::db::connect(args),
        cli::args::Command::Ssh(args) => cli::ssh::connect(args),
    }
}
