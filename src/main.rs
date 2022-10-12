#![allow(unused)]
use clap::{Parser, arg};
use gru::Configuration;
use std::{env, process, error::Error, fs};
use serde::Deserialize;

/// Config file manager
static CONFIG_FILE_NAME: &str = "gru-settings.json";
pub struct Repository {
    pub path: String
}

pub enum Command {
    Check,
    Install,
    Apply
}

pub struct File<'a> {
    local_path: &'a str,
    repo_path: &'a str
}

fn main() {
    let command = read_command(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });
    let content = gru::read_config(CONFIG_FILE_NAME).unwrap_or_else(|err| {
        eprintln!("Problem reading configuration file: {err}");
        process::exit(1);
    });
    let config = Configuration::new(&content).unwrap_or_else(|err| {
        eprintln!("Problem with creating configuration object: {err}");
        process::exit(1);
    });

    println!("Repository url: {}", config.url);
}

pub fn read_command(mut args: env::Args) -> Result<Command, Box<dyn Error>> {
    clap::Command::new("git")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            clap::Command::new("clone")
            .about("Clones repos")
            .arg(arg!(<REMOTE> "The remote to clone"))
            .arg_required_else_help(true),
        );
}