use clap::{Parser, Subcommand, Args};
use crate::config::get_config;
use crate::file_ops::parse_file;
use std::process;

#[derive(Debug, Parser)]
pub struct GruArgs {
    /// Manage configuration files 
    #[clap(subcommand)]
    pub action: GruCommand,
}

#[derive(Debug, Subcommand)]
pub enum GruCommand {
    /// Check differences between files
    Check(CheckSubcommand),
    /// Install files in filesystem
    Install(InstallSubcommand),
    /// Apply changes to repository
    Apply(ApplySubcommand),
}

#[derive(Debug, Args)]
pub struct CheckSubcommand {
    /// Configuration file path 
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct InstallSubcommand {
    /// Configuration file path 
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct ApplySubcommand {
    /// Configuration file path 
    pub file: Option<String>,
}

pub fn run_check(subcommand: CheckSubcommand) {
    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {}", err);
        process::exit(1);
    });
    let mut installed_files = 0;
    for line in config.files.iter() {
        parse_file(line, &mut installed_files);
    }
    println!("Files missing from file system: {}", installed_files);
}

pub fn run_install(_subcommand: InstallSubcommand) {
    // Implementation for install
}

pub fn run_apply(_subcommand: ApplySubcommand) {
    // Implementation for apply
}
