mod config;
mod commands;
mod file_ops;

use clap::Parser;
use commands::{run_apply, run_check, run_install, GruArgs, GruCommand};

fn main() {
    let command = GruArgs::parse();
    match command.action {
        GruCommand::Check(subcommand) => run_check(subcommand),
        GruCommand::Install(subcommand) => run_install(subcommand),
        GruCommand::Apply(subcommand) => run_apply(subcommand),
    }
}
