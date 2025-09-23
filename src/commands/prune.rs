use crate::model::BooksDb;
use anyhow::Result;
use tracing::info;

pub fn cmd_prune(db: &mut BooksDb) -> Result<()> {
    let before = db.books.len();
    db.books.retain(|b| !b.stale);
    info!(
        "Pruned {} stale record(s)",
        before.saturating_sub(db.books.len())
    );
    Ok(())
}
