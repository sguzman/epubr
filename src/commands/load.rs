use anyhow::Result;
use rayon::prelude::*;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::hash;
use crate::metadata;
use crate::model::{BookEntry, BooksDb};
use crate::scan::gather_epubs;
use crate::util::{file_uri, now_iso8601};

pub fn cmd_load(
    db: &mut BooksDb,
    _db_path: &PathBuf,
    root_dir: PathBuf,
    follow_symlinks: bool,
) -> Result<()> {
    let epubs = gather_epubs(&root_dir, follow_symlinks)?;
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

            let xxhash = hash::xxh3_file(&full_path).ok();
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

    for mut e in new_entries {
        merge_entry(db, &mut e);
    }

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
            }
            _ => {
                existing.stale = true;
                existing.missing = true;
                db.books.push(new.clone());
                // NOTE: per your request, "Inserted ..." is debug; updated stays info.
                tracing::info!("Updated (stale→new): {}", new.full_path);
            }
        }
    } else {
        db.books.push(new.clone());
        tracing::debug!("Inserted: {}", new.full_path); // downgraded to debug
    }
}
