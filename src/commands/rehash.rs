use anyhow::Result;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::hash;
use crate::model::BooksDb;

/// Fill in missing hashes (xxhash == None) for entries whose files exist.
/// If `force` is true, recompute all hashes for existing files.
pub fn cmd_rehash(db: &mut BooksDb, force: bool) -> Result<()> {
    // Gather candidate indices + paths to avoid borrowing issues
    let candidates: Vec<(usize, PathBuf)> = db
        .books
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            // Skip stale entries; focus on the current view of the library
            if b.stale {
                return None;
            }
            let path = PathBuf::from(&b.full_path);
            if !path.exists() {
                // keep DB flag truthful; don't try to hash missing files
                // (do not modify here; a future `check` will mark missing if needed)
                return None;
            }
            if force || b.xxhash.is_none() {
                Some((i, path))
            } else {
                None
            }
        })
        .collect();

    if candidates.is_empty() {
        info!("rehash: nothing to do (no eligible entries).");
        return Ok(());
    }

    info!(
        "rehash: processing {} eligible entr(y/ies){}",
        candidates.len(),
        if force { " (force mode)" } else { "" }
    );

    // Hash in parallel; also fetch up-to-date size while we're at it
    #[derive(Debug)]
    struct RehashOut {
        idx: usize,
        size_bytes: u64,
        hash: Option<u128>,
    }

    let results: Vec<RehashOut> = candidates
        .into_par_iter()
        .map(|(idx, path)| {
            let size_bytes = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            let hash = match hash::xxh3_file(&path) {
                Ok(h) => Some(h),
                Err(e) => {
                    warn!("rehash: failed to hash {}: {}", path.display(), e);
                    None
                }
            };
            RehashOut {
                idx,
                size_bytes,
                hash,
            }
        })
        .collect();

    // Apply updates serially to the DB
    let mut updated = 0usize;
    for r in results {
        if let Some(entry) = db.books.get_mut(r.idx) {
            // Update size to current value (cheap & useful)
            entry.size_bytes = r.size_bytes;
            // Only write hash if we got one; otherwise leave as-is
            if r.hash.is_some() {
                entry.xxhash = r.hash;
                updated += 1;
            }
        }
    }

    info!("rehash: updated {} entr(y/ies).", updated);
    Ok(())
}
