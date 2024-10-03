mod config;
mod commands;
mod file_ops;

use clap::Parser;
use commands::{run_apply, run_check, run_install, DotccArgs, DotccCommand};

fn main() {
    let command = DotccArgs::parse();
    match command.action {
        DotccCommand::Check(subcommand) => run_check(subcommand),
        DotccCommand::Install(subcommand) => run_install(subcommand),
        DotccCommand::Apply(subcommand) => run_apply(subcommand),
    }
}
