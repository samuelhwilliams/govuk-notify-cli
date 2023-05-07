mod cli;

use std::io;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, "notify", &mut io::stdout());
}

fn main() {
    let args = cli::args::NotifyCliArgs::parse();

    if let Some(generator) = args.generator {
        let mut cmd = cli::args::NotifyCliArgs::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        match args.command {
            Some(cli::args::Command::Aws(command)) => match command.subcommand {
                cli::args::AwsSubcommand::Exec(args) => cli::aws::exec(args),
                cli::args::AwsSubcommand::Console(args) => cli::aws::console(args),
            },
            Some(cli::args::Command::Db(args)) => cli::db::connect(args),
            Some(cli::args::Command::Ssh(args)) => cli::ssh::connect(args),
            None => {}
        }
    }
}
