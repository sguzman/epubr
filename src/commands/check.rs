use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::hash;
use crate::metadata;
use crate::model::{BookEntry, BooksDb};
use crate::util::{file_uri, now_iso8601};

pub fn cmd_check(db: &mut BooksDb) -> Result<()> {
    check_and_update(db)
}

/// --check pass:
/// - If file missing → set missing=true
/// - If file present but hash differs → mark existing stale+missing and create a new entry
fn check_and_update(db: &mut BooksDb) -> Result<()> {
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
        if let Ok(new_hash) = hash::xxh3_file(&path) {
            match (&existing.xxhash, new_hash) {
                (Some(old), neu) if *old == neu => {
                    // unchanged
                }
                (_, neu) => {
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
            existing.missing = true;
            warn!("Unreadable, marked missing: {}", existing.full_path);
        }
    }

    db.books.extend(to_push);
    Ok(())
}
