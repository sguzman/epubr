mod args;
mod db;
mod hash;
mod metadata;
mod model;
mod scan;
mod util;

use crate::args::Cli;
use crate::db::{BooksDb, load_db, save_db};
use crate::model::{BookEntry, Verbosity};
use crate::scan::gather_epubs;
use crate::util::{file_uri, now_iso8601};
use anyhow::Result;
use clap::Parser;
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;
use tracing::{debug, info, warn};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Logging
    {
        let level = match cli.verbose {
            Verbosity::Quiet => "warn",
            Verbosity::Info => "info",
            Verbosity::Debug => "debug",
        };
        tracing_subscriber::fmt()
            .with_env_filter(level)
            .with_target(false)
            .init();
    }

    // Threads
    if cli.threads > 0 {
        ThreadPoolBuilder::new()
            .num_threads(cli.threads)
            .build_global()
            .ok();
        info!("Using {} thread(s)", cli.threads);
    }

    let root = cli.root.clone();
    let db_path = cli.db.clone();

    let mut db: BooksDb = load_db(&db_path).unwrap_or_default();
    info!("Loaded DB with {} record(s)", db.books.len());

    if cli.check {
        check_and_update(&mut db)?;
    }

    if cli.prune {
        let before = db.books.len();
        db.books.retain(|b| !b.stale);
        info!(
            "Pruned {} stale record(s)",
            before.saturating_sub(db.books.len())
        );
    }

    // Main scan (if user provided a root directory)
    if let Some(root_dir) = root {
        let epubs = gather_epubs(&root_dir, cli.follow_symlinks)?;
        info!("Found {} epub file(s)", epubs.len());

        let found_at = now_iso8601();

        let new_entries: Vec<BookEntry> = epubs
            .into_par_iter()
            .map(|path| {
                let full_path = path.canonicalize().unwrap_or(path.clone());
                let filename = full_path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown.epub".to_string());
                let uri = file_uri(&full_path);
                let protocol = "file".to_string();

                // Hash file
                let xxhash = hash::xxh3_file(&full_path).ok();

                // Metadata (best-effort)
                let meta = metadata::extract_epub_metadata(&full_path).unwrap_or_default();

                BookEntry {
                    full_path: full_path.to_string_lossy().to_string(),
                    uri_path: uri,
                    protocol,
                    filename,
                    xxhash,
                    date_found: found_at.clone(),
                    missing: false,
                    stale: false,
                    title: meta.title,
                    author: meta.author,
                    description: meta.description,
                    chapters: meta.chapters,
                    publish_date: meta.publish_date,
                    publisher: meta.publisher,
                    other_metadata: meta.other_metadata,
                }
            })
            .collect();

        // Merge into DB (dataflow-y, pure-ish helpers)
        for mut e in new_entries {
            merge_entry(&mut db, &mut e);
        }
    }

    // Stow (stub)
    if cli.stow {
        warn!("--stow is not implemented yet (will zpaq ultra-compress to a local archive)");
    }

    // Save
    save_db(&db_path, &db)?;
    info!("Saved DB to {}", db_path.to_string_lossy());

    Ok(())
}

/// Merge logic:
/// - If an entry with the same full_path exists:
///     - If hash unchanged (or missing both), do nothing.
///     - If hash changed, mark old as stale+missing, insert new as fresh.
/// - If path doesn't exist, insert new.
fn merge_entry(db: &mut BooksDb, new: &mut BookEntry) {
    if let Some(existing) = db
        .books
        .iter_mut()
        .find(|b| b.full_path == new.full_path && !b.stale)
    {
        match (&existing.xxhash, &new.xxhash) {
            (Some(old), Some(neu)) if old == neu => {
                debug!("Unchanged: {}", new.full_path);
                // Optionally refresh metadata or timestamps here
            }
            _ => {
                existing.stale = true;
                existing.missing = true;
                db.books.push(new.clone());
                info!("Updated (stale→new): {}", new.full_path);
            }
        }
    } else {
        db.books.push(new.clone());
        info!("Inserted: {}", new.full_path);
    }
}

/// --check pass:
/// - If file missing → set missing=true
/// - If file present but hash differs → mark existing stale+missing and create a new entry
fn check_and_update(db: &mut BooksDb) -> Result<()> {
    // Collect changes, apply after to avoid borrow thrash
    let mut to_push: Vec<BookEntry> = Vec::new();

    for existing in db.books.iter_mut().filter(|b| !b.stale) {
        let path = PathBuf::from(&existing.full_path);
        if !path.exists() {
            if !existing.missing {
                existing.missing = true;
                info!("Marked missing: {}", existing.full_path);
            }
            continue;
        }
        // Re-hash to detect content changes
        if let Ok(new_hash) = hash::xxh3_file(&path) {
            match (&existing.xxhash, new_hash) {
                (Some(old), neu) if *old == neu => {
                    debug!("OK (unchanged): {}", existing.full_path);
                }
                (_, neu) => {
                    // Mark old stale+missing and create a fresh entry
                    existing.stale = true;
                    existing.missing = true;

                    let meta = metadata::extract_epub_metadata(&path).unwrap_or_default();
                    let found_at = now_iso8601();
                    let uri = file_uri(&path);
                    let filename = path
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown.epub".to_string());

                    let fresh = BookEntry {
                        full_path: existing.full_path.clone(),
                        uri_path: uri,
                        protocol: "file".to_string(),
                        filename,
                        xxhash: Some(neu),
                        date_found: found_at,
                        missing: false,
                        stale: false,
                        title: meta.title,
                        author: meta.author,
                        description: meta.description,
                        chapters: meta.chapters,
                        publish_date: meta.publish_date,
                        publisher: meta.publisher,
                        other_metadata: meta.other_metadata,
                    };
                    to_push.push(fresh);
                    info!("Changed → new record: {}", existing.full_path);
                }
            }
        } else {
            // Couldn’t read for some reason; mark missing
            existing.missing = true;
            warn!("Unreadable, marked missing: {}", existing.full_path);
        }
    }

    db.books.extend(to_push);
    Ok(())
}
