use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::hash;
use crate::metadata;
use crate::model::{BookEntry, BooksDb, FileFormat};
use crate::util::{file_uri, now_iso8601};

pub fn cmd_check(db: &mut BooksDb) -> Result<()> {
    check_and_update(db)
}

fn check_and_update(db: &mut BooksDb) -> Result<()> {
    let mut to_push = Vec::new();

    for existing in db.books.iter_mut().filter(|e| !e.stale) {
        let path = PathBuf::from(&existing.full_path);
        if let Ok(md) = fs::metadata(&path) {
            let size = md.len();
            let new_hash = hash::xxh3_file(&path).ok();
            if existing.xxhash != new_hash {
                let meta = match existing.format {
                    FileFormat::Epub => metadata::extract_epub_metadata(&path).unwrap_or_default(),
                    FileFormat::Pdf => metadata::extract_pdf_metadata(&path).unwrap_or_default(),
                };
                let fresh = BookEntry {
                    full_path: existing.full_path.clone(),
                    uri_path: file_uri(&path),
                    protocol: existing.protocol.clone(),
                    filename: existing.filename.clone(),
                    xxhash: new_hash,
                    date_found: now_iso8601(),
                    missing: false,
                    stale: false,
                    size_bytes: size,
                    format: existing.format,
                    title: meta.title,
                    author: meta.author,
                    description: meta.description,
                    chapters: meta.chapters,
                    publish_date: meta.publish_date,
                    publisher: meta.publisher,
                    other_metadata: meta.other_metadata,
                };
                existing.stale = true;
                existing.missing = false;
                to_push.push(fresh);
                info!("Changed → new record: {}", existing.full_path);
            } else {
                // update size if needed
                existing.size_bytes = size;
            }
        } else {
            existing.missing = true;
            warn!("Unreadable, marked missing: {}", existing.full_path);
        }
    }

    db.books.extend(to_push);
    Ok(())
}
