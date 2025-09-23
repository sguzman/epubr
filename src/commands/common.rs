use crate::model::{BookEntry, BooksDb};
use tracing::{debug, info};

/// Merge logic used by `load` and `merge`:
/// - If an entry with the same full_path exists and hash unchanged → no-op
/// - If same path but hash changed → mark old stale+missing, push new
/// - If path not present → insert new
pub fn merge_entry(db: &mut BooksDb, new: &mut BookEntry) {
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
                info!("Updated (stale→new): {}", new.full_path);
            }
        }
    } else {
        db.books.push(new.clone());
        debug!("Inserted: {}", new.full_path); // debug per your preference
    }
}
