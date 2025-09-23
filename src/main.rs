mod args;
mod commands;
mod db;
mod hash;
mod log;
mod metadata;
mod model;
mod scan;
mod util;

use anyhow::Result;
use clap::Parser;

use crate::args::Cli;
use crate::log as elog; // avoid name clash
use crate::model::Verbosity;

fn main() -> Result<()> {
    // Parse CLI
    let cli = Cli::parse();

    // Logging (color, timestamps) in one place
    elog::init(match cli.verbose {
        Verbosity::Quiet => "warn",
        Verbosity::Info => "info",
        Verbosity::Debug => "debug",
    });

    // Hand off to command runner (threads, db load/save, dispatch)
    commands::run(cli)
}
