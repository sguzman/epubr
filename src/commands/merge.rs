use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

use crate::commands::common::merge_entry;
use crate::db::load_db;
use crate::model::BooksDb;
use crate::util::now_iso8601;

/// Merge another JSON DB (produced by this program) into the current DB.
/// - Entries are merged using the same rules as `load` (by full_path + xxhash).
/// - If the other DB has entries for paths you don't have, they are added.
/// - If both have the same path but content changed, the old one becomes stale+missing.
pub fn cmd_merge(db: &mut BooksDb, other_db_path: PathBuf) -> Result<()> {
    let other = load_db(&other_db_path).unwrap_or_default();
    let n = other.books.len();

    // Treat the other DB's entries as incoming "candidates"
    for mut e in other.books.into_iter() {
        // Ensure date_found is set
        if e.date_found.is_empty() {
            e.date_found = now_iso8601();
        }
        // Merge into current DB
        merge_entry(db, &mut e);
    }

    info!(
        "Merged {} record(s) from {}",
        n,
        other_db_path.to_string_lossy()
    );
    Ok(())
}
