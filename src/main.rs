mod cli;

use clap::Parser;

fn main() {
    let args = cli::args::NotifyCliArgs::parse();

    match args.command {
        cli::args::Command::Aws(command) => match command.subcommand {
            cli::args::AwsSubcommand::Exec(ec) => cli::aws::exec(ec),
            cli::args::AwsSubcommand::Console(ec) => cli::aws::console(ec),
        },
        cli::args::Command::Db(ec) => cli::db::connect(ec),
        cli::args::Command::Ssh(_ec) => {}
    }
}
