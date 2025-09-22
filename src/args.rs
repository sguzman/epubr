use crate::model::Verbosity;
use clap::Parser;
use std::path::PathBuf;

/// EPUB indexer
#[derive(Debug, Parser, Clone)]
#[command(author, version, about)]
pub struct Cli {
    /// Root directory to scan (recursively) for .epub files
    #[arg(value_name = "DIR")]
    pub root: Option<PathBuf>,

    /// Path to books.json
    #[arg(long, value_name = "FILE", default_value = "books.json")]
    pub db: PathBuf,

    /// Number of threads to use (rayon). 0 = use default.
    #[arg(long = "threads", short = 't', default_value_t = 0)]
    pub threads: usize,

    /// Verbosity: 0=quiet, 1=info, 2=debug
    #[arg(long = "verbose", short = 'v', value_enum, default_value_t = Verbosity::Info)]
    pub verbose: Verbosity,

    /// Check current DB against filesystem (mark missing/changed)
    #[arg(long)]
    pub check: bool,

    /// Remove stale entries from DB
    #[arg(long)]
    pub prune: bool,

    /// Follow symlinks when walking directories
    #[arg(long)]
    pub follow_symlinks: bool,

    /// Make a local zpaq ultra-compressed archive of found epubs (stub)
    #[arg(long)]
    pub stow: bool,

    /// Serve a database (ignore for now)
    #[arg(long, value_name = "DB")]
    pub serve: Option<String>,

    /// Local interactive query mode (ignore for now)
    #[arg(long)]
    pub query: bool,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}
