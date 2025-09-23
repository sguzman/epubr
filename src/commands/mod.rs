pub mod check;
pub mod common;
pub mod load;
pub mod merge;
pub mod prune;

use anyhow::Result;
use rayon::ThreadPoolBuilder;
use tracing::{info, warn};

use crate::args::{Cli, Commands};
use crate::db::{load_db, save_db};
use crate::model::BooksDb;

pub fn run(cli: Cli) -> Result<()> {
    // Threading
    if cli.threads > 0 {
        ThreadPoolBuilder::new()
            .num_threads(cli.threads)
            .build_global()
            .ok();
        info!("Using {} thread(s)", cli.threads);
    }

    // Load DB
    let db_path = cli.db.clone();
    let mut db: BooksDb = load_db(&db_path).unwrap_or_default();
    info!("Loaded DB with {} record(s)", db.books.len());

    match cli.cmd {
        Commands::Load {
            root,
            follow_symlinks,
        } => {
            load::cmd_load(&mut db, &db_path, root, follow_symlinks)?;
            save_db(&db_path, &db)?;
            info!("Saved DB to {}", db_path.to_string_lossy());
        }

        Commands::Check => {
            check::cmd_check(&mut db)?;
            save_db(&db_path, &db)?;
            info!("Saved DB to {}", db_path.to_string_lossy());
        }

        Commands::Prune => {
            prune::cmd_prune(&mut db)?;
            save_db(&db_path, &db)?;
            info!("Saved DB to {}", db_path.to_string_lossy());
        }

        Commands::Merge { other } => {
            merge::cmd_merge(&mut db, other)?;
            save_db(&db_path, &db)?;
            info!("Saved DB to {}", db_path.to_string_lossy());
        }

        Commands::Serve { backend } => {
            warn!(
                "`serve` is not implemented yet (DNI). Backend arg = {:?}",
                backend
            );
        }

        Commands::Query => {
            warn!("`query` is not implemented yet (DNI).");
        }

        Commands::Stow => {
            warn!("`stow` is not implemented yet (DNI). Will zpaq ultra-compress epubs.");
        }
    }

    Ok(())
}
