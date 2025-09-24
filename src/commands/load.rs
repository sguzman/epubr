use anyhow::Result;
use rayon::prelude::*;
use std::path::PathBuf;
use tracing::info;

use crate::commands::common::merge_entry;
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
    no_hash: bool,
) -> Result<()> {
    let epubs = gather_epubs(&root_dir, follow_symlinks)?;
    info!("Found {} epub file(s)", epubs.len());

    let found_at = now_iso8601();

    let new_entries: Vec<BookEntry> = epubs
        .into_par_iter()
        .map(|path| {
            let full_path = path.canonicalize().unwrap_or(path.clone());
            // File size (bytes)
            let size_bytes = fs::metadata(&full_path).map(|m| m.len()).unwrap_or(0);

            let filename = full_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown.epub".to_string());
            let uri = file_uri(&full_path);
            let protocol = "file".to_string();

            // Conditional hash
            let xxhash = if no_hash {
                None
            } else {
                hash::xxh3_file(&full_path).ok()
            };
            let meta = metadata::extract_epub_metadata(&full_path).unwrap_or_default();

            BookEntry {
                full_path: full_path.to_string_lossy().to_string(),
                uri_path: uri,
                protocol,
                filename,
                size_bytes,
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
