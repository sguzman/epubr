use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use tracing::info;

use crate::commands::common::merge_entry;
use crate::hash;
use crate::metadata;
use crate::model::{BookEntry, BooksDb, FileFormat};
use crate::scan::gather_epubs;
use crate::util::{file_uri, now_iso8601};

pub fn cmd_load(
    db: &mut BooksDb,
    _db_path: &PathBuf,
    root: PathBuf,
    follow_symlinks: bool,
    no_hash: bool,
) -> Result<()> {
    let files = gather_epubs(&root, follow_symlinks)?; // now includes .epub and .pdf
    info!("found {} candidate file(s)", files.len());

    // Tally what we *found* by extension upfront
    let mut found_epub: usize = 0;
    let mut found_pdf: usize = 0;

    let new_entries: Vec<BookEntry> = files
        .par_iter()
        .map(|p| {
            let size = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
            let ext_lc = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_ascii_lowercase())
                .unwrap_or_default();

            let format = if ext_lc == "pdf" {
                FileFormat::Pdf
            } else {
                FileFormat::Epub
            };

            // Count found by format
            match format {
                FileFormat::Epub => {
                    // increment later once we’re back on the main thread to avoid locks,
                    // but we can’t here—so we’ll do a second pass below if needed.
                    // Instead, tally in a post-collect scan:
                }
                FileFormat::Pdf => {}
            }

            let meta = match format {
                FileFormat::Epub => metadata::extract_epub_metadata(p).unwrap_or_default(),
                FileFormat::Pdf => metadata::extract_pdf_metadata(p).unwrap_or_default(),
            };

            let hash = if no_hash {
                None
            } else {
                hash::xxh3_file(p).ok()
            };

            BookEntry {
                full_path: p.to_string_lossy().to_string(),
                uri_path: file_uri(p),
                protocol: "file".into(),
                filename: p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default()
                    .to_string(),
                xxhash: hash,
                date_found: now_iso8601(),
                missing: false,
                stale: false,
                size_bytes: size,
                format,
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

    // Now that we have formats, tally the “found” counts
    for e in &new_entries {
        match e.format {
            FileFormat::Epub => found_epub += 1,
            FileFormat::Pdf => found_pdf += 1,
        }
    }

    // Tally what we actually *added* after merge (as opposed to updated)
    let mut added_epub: usize = 0;
    let mut added_pdf: usize = 0;

    for mut e in new_entries {
        let before = db.books.len();
        merge_entry(db, &mut e);
        let after = db.books.len();
        if after > before {
            match e.format {
                FileFormat::Epub => added_epub += 1,
                FileFormat::Pdf => added_pdf += 1,
            }
        }
    }

    // Final summary log
    let found_total = found_epub + found_pdf;
    let added_total = added_epub + added_pdf;
    info!(
        "load summary → found: {} (epub={}, pdf={}); added: {} (epub={}, pdf={})",
        found_total, found_epub, found_pdf, added_total, added_epub, added_pdf
    );

    Ok(())
}
