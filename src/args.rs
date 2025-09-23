use crate::model::Verbosity;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// EPUB indexer
#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to books.json (global)
    #[arg(long, value_name = "FILE", default_value = "books.json")]
    pub db: PathBuf,

    /// Number of threads to use (rayon). 0 = use default. (global)
    #[arg(long = "threads", short = 't', default_value_t = 0)]
    pub threads: usize,

    /// Verbosity: 0=quiet, 1=info, 2=debug (global)
    #[arg(long = "verbose", short = 'v', value_enum, default_value_t = Verbosity::Info)]
    pub verbose: Verbosity,

    /// Command to run
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Scan a directory tree for .epub files and load/update the DB
    Load {
        /// Root directory to scan (recursively) for .epub files
        #[arg(value_name = "DIR")]
        root: PathBuf,

        /// Follow symlinks when walking directories
        #[arg(long)]
        follow_symlinks: bool,
    },

    /// Check DB entries against the filesystem (mark missing/changed)
    Check,

    /// Remove stale entries from the DB
    Prune,

    /// Serve a DB (DNI / stub)
    Serve {
        /// Optional backend name (e.g., meilisearch, surrealdb, ...)
        #[arg(value_name = "BACKEND")]
        backend: Option<String>,
    },

    /// Local interactive query mode (DNI / stub)
    Query,

    /// Create a zpaq archive of epubs (DNI / stub)
    Stow,
}
